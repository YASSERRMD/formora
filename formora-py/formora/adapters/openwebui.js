/**
 * Open WebUI Formora Adapter
 *
 * This adapter integrates Formora forms with Open WebUI.
 * It injects the form data into the chat textarea and triggers the send button.
 *
 * INSTALLATION:
 * Add this script to Open WebUI's custom JavaScript section or inject via the browser.
 *
 * USAGE:
 * The adapter automatically detects form submissions and injects the data into chat.
 */

window.addEventListener('formora:submit', (e) => {
  // Find Open WebUI's textarea
  const textarea = document.querySelector('textarea[placeholder*="Message"]') ||
                   document.querySelector('#chat-input') ||
                   document.querySelector('textarea');

  if (textarea) {
    // Inject the formora message
    textarea.value = e.detail.raw;
    textarea.dispatchEvent(new Event('input', { bubbles: true }));
    textarea.dispatchEvent(new Event('change', { bubbles: true }));

    // Find and click the send button
    const sendButton = document.querySelector('button[aria-label*="Send"]') ||
                       document.querySelector('button svg[class*="send"]')?.closest('button') ||
                       document.querySelector('button[type="submit"]');

    if (sendButton) {
      sendButton.click();
    }
  }
});

/**
 * TESTED VERSION: Open WebUI (latest as of 2025)
 */
