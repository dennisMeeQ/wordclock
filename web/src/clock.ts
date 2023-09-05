// import { memory } from 'wordclock_wasm/wordclock_bg';
import init, { get_current_pattern, Pattern } from 'wordclock_wasm';

export async function setupClock(element: HTMLButtonElement) {
  await init();

  const pattern = get_current_pattern();
  const clock = renderClock(pattern);

  element.innerHTML = `
  <div class="clock"> 
    ${clock}
  </div>`;
}

const renderClock = (pattern: Pattern) => {
  const width = pattern.width();
  const height = pattern.height();

  let result = '';

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const letter = pattern.get_field_letter(x, y);

      const className = `field ${
        pattern.get_field_status(x, y) ? 'on' : 'off'
      }`;

      result = result.concat(`<pre class="${className}">${letter}</pre>`);
    }
  }

  return result;
};
