/**
 * Chainlit Formora Adapter
 *
 * This adapter integrates Formora forms with Chainlit.
 * It uses Chainlit's window.sendMessage API if available, with fallback to DOM injection.
 *
 * INSTALLATION:
 * Add this script to your Chainlit app's custom JavaScript or inject via the browser.
 *
 * USAGE:
 * The adapter automatically detects form submissions and sends the message.
 */

window.addEventListener('formora:submit', (e) => {
  // Try Chainlit's API first
  if (window.sendMessage && typeof window.sendMessage === 'function') {
    window.sendMessage(e.detail.raw);
    return;
  }

  // Fallback: Find the chat input
  const textarea = document.querySelector('textarea[placeholder*="Message"]') ||
                   document.querySelector('#chat-input') ||
                   document.querySelector('textarea');

  if (textarea) {
    // Inject the formora message
    textarea.value = e.detail.raw;
    textarea.dispatchEvent(new Event('input', { bubbles: true }));
    textarea.dispatchEvent(new Event('change', { bubbles: true }));

    // Trigger Enter key to send
    textarea.dispatchEvent(new KeyboardEvent('keydown', {
      key: 'Enter',
      code: 'Enter',
      keyCode: 13,
      which: 13,
      bubbles: true,
    }));
  }
});

/**
 * TESTED VERSION: Chainlit (latest as of 2025)
 */
