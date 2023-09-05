import './style.css';
import { setupClock } from './clock.ts';

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>  
    <div id="clock" class="wrapper"/>
  </div>
`;

while (true) {
  setupClock(document.querySelector<HTMLButtonElement>('#clock')!);

  await sleep(30000);
}
