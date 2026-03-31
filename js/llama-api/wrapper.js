const express = require('express');
const bodyParser = require('body-parser');
const fetch = require('node-fetch');

const app = express();
app.use(bodyParser.json());

const LLAMA_SERVER = 'http://127.0.0.1:8080'; // your running llama-server
const MODELS = {
    'local-llama': 'local-llama' // just keep names consistent with llama-server
};

app.get('/v1/models', async (req, res) => {
  try {
    const response = await fetch(`${LLAMA_SERVER_URL}/v1/models`);
    const data = await response.json();
    res.json(data);
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Failed to fetch models from llama-server' });
  }
});

app.post('/api/chat',async(req,res)=>{
const body={...req.body,stream:false}
const r=await fetch(`${LLAMA_SERVER}/v1/chat/completions`,{
method:'POST',
headers:{'Content-Type':'application/json'},
body:JSON.stringify(body)
})
const j=await r.json()
res.json({
model:j.model,
created_at:new Date().toISOString(),
message:j.choices[0].message,
done:true
})
})

app.listen(9000, () => {
  console.log('Local Ollama-compatible proxy running on http://localhost:9000');
});
