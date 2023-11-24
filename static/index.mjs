import {h, render} from "http://unpkg.com/preact?module";
import htm from "http://unpkg.com/htm?module";

const html = htm.bind(h);

function Cpu({name, usage}) {
    return (html`
        <div class="bar">
            <div class="bar-inner" style="width: ${usage}%"></div>
            <span class="label">${name.toUpperCase()}</span>
            <span class="label">${usage.toFixed(2)}%</span>
        </div>`);
}

function Cpus({cpus}) {
    return html`
        <div class="bar-container">
            ${cpus.map((cpu) => html`
                <${Cpu} ...${cpu}></${Cpu}>`)}
        </div>`;
}

function Memory({total, free, used}) {
    const usage = used / total;
    const usedGb = used / 1024 / 1024 / 1024;
    const usedGbStr = usedGb.toFixed(2);
    const freeGb = free / 1024 / 1024 / 1024;
    const freeGbStr = freeGb.toFixed(2);
    const totalGb = total / 1024 / 1024 / 1024;
    const totalGbStr = totalGb.toFixed(2);
    return html`
        <div>
            <div class="bar-container">
                <div class="bar" style="flex: 1">
                    <div class="bar-inner" style="width: ${usage * 100}%"></div>
                    <span class="label">Used: ${usedGbStr} / ${totalGbStr} GB</span>
                    <span class="label">Free: ${freeGbStr} GB</span>
                </div>
            </div>
        </div>`
}

function App({cpus, memory}) {
    return html`
        <div class="container">
            <h2>CPU</h2>
            <${Cpus} cpus=${cpus}></${Cpus}>
            <h2>Memory</h2>
            <${Memory} ...${memory}></${Memory}>
        </div>`;
}

const url = new URL("/realtime", window.location.href);
url.protocol = url.protocol.replace("http", "ws");

const ws = new WebSocket(url.href);
ws.onmessage = (ev) => {
    const json = JSON.parse(ev.data);
    render(html`
        <${App} ...${json}></${App}>`, document.body);
};
