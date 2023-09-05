import './style.css';
import { setupClock } from './clock.ts';

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>  
    <div id="greeting" class="card" />
  </div>
`;

while (true) {
  setupClock(document.querySelector<HTMLButtonElement>('#greeting')!);

  await sleep(30000);
}
