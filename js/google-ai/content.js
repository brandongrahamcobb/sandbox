var host = "https://google.com/search?q"
const regex = /[^\w*:\/\/\w*\.google\.com\/search.?q].*/gi;
const ai = "%20-ai"
if (location.href.match(regex) && !(location.href.includes(ai))) {
    chrome.runtime.sendMessage({redirect: host + location.href.match(regex) + "%20-ai"});
}
