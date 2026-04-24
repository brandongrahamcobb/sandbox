// wrapper.cpp
#include "httplib.h"
#include <llama.h>
#include <mutex>
#include <vector>
#include <string>
#include <sstream>
#include <thread>
#include <stdexcept>

struct LlamaWrapper {
    llama_model* model = nullptr;
    llama_context* ctx = nullptr;
    std::mutex cache_mutex;
    std::vector<llama_token> session_tokens;
    size_t max_session_tokens = 8192;

    void init(const std::string& model_path) {
        llama_model_params model_params = llama_model_default_params();
        model = llama_model_load_from_file(model_path.c_str(), model_params);
        if (!model) throw std::runtime_error("Failed to load model");

        llama_context_params ctx_params = llama_context_default_params();
        ctx = llama_init_from_model(model, ctx_params);
        if (!ctx) throw std::runtime_error("Failed to initialize context");
    }

    std::vector<llama_token> tokenize_text(const std::string& text) {
        std::vector<llama_token> tokens(2048);
        int n_tokens = llama_tokenize(ctx->model->vocab, text.c_str(), (int)text.size(),
                                      tokens.data(), (int)tokens.size(), true, true);
        tokens.resize(n_tokens);
        return tokens;
    }

    std::string detokenize(const std::vector<llama_token>& tokens) {
        std::string output;
        char buf[16];
        for (llama_token tok : tokens) {
            int n = llama_token_to_str(ctx, tok, buf, sizeof(buf));
            output += std::string(buf, n);
        }
        return output;
    }

    std::string generate(const std::string& prompt, int max_tokens = 256) {
        std::lock_guard<std::mutex> lock(cache_mutex);

        auto input_tokens = tokenize_text(prompt);
        session_tokens.insert(session_tokens.end(), input_tokens.begin(), input_tokens.end());

        int start_pos = 0;
        int n_threads = std::thread::hardware_concurrency();
        llama_eval(ctx, session_tokens.data(), (int)session_tokens.size(), start_pos, n_threads);

        std::vector<llama_token> output_tokens;
        for (int i = 0; i < max_tokens; ++i) {
            llama_token tok = llama_sample_top_p_top_k(ctx, 0, 0.7f, 1.0f); // same as server.cpp
            if (tok == 0) break;
            session_tokens.push_back(tok);
            output_tokens.push_back(tok);
        }

        if (session_tokens.size() > max_session_tokens) {
            session_tokens.erase(session_tokens.begin(),
                                 session_tokens.begin() + (session_tokens.size() - max_session_tokens));
        }

        return detokenize(output_tokens);
    }

    void add_to_session(const std::string& message, const std::string& role) {
        std::lock_guard<std::mutex> lock(cache_mutex);
        auto tokens = tokenize_text(role + ": " + message);
        session_tokens.insert(session_tokens.end(), tokens.begin(), tokens.end());
    }

    void reset_cache() {
        std::lock_guard<std::mutex> lock(cache_mutex);
        session_tokens.clear();
    }
};

int main() {
    LlamaWrapper llama;
    llama.init("Qwen3.5-4B-Q4_K_M.gguf");

    httplib::Server svr;

    svr.Post("/generate", [&](const httplib::Request& req, httplib::Response& res) {
        auto prompt = req.get_param_value("prompt");
        res.set_content(llama.generate(prompt), "text/plain");
    });

    svr.Post("/chat", [&](const httplib::Request& req, httplib::Response& res) {
        auto role = req.get_param_value("role");
        auto content = req.get_param_value("content");
        llama.add_to_session(content, role);
        res.set_content(llama.generate(content), "text/plain");
    });

    svr.Post("/reset", [&](const httplib::Request& req, httplib::Response& res) {
        llama.reset_cache();
        res.set_content("Session reset", "text/plain");
    });

    svr.listen("localhost", 8080);
}
