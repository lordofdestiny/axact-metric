import { h, render } from "http://unpkg.com/preact?module";
import htm from "http://unpkg.com/htm?module";

const html = htm.bind(h);

function App(props) {
  return html`<div class="container">
  <h2>CPU</h2>
    <div class="cpus">
      ${props.snap.cpu.map(([name, usage]) => {
        return html`<div class="bar">
          <div class="bar-inner" style="width: ${usage}%"></div>
          <span class="label">${name.toUpperCase()}</span>
          <span class="label">${usage.toFixed(2)}%</span>
        </div>`;
      })}
    </div>
  </div>`;
}

const url = new URL("/realtime", window.location.href);
url.protocol = url.protocol.replace("http", "ws");

const ws = new WebSocket(url.href);
ws.onmessage = (ev) => {
  const json = JSON.parse(ev.data);
  render(html`<${App} snap=${json}></${App}>`, document.body);
};
