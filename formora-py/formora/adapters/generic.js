/**
 * Generic Formora Adapter
 *
 * This adapter works with any chat UI that allows custom JavaScript.
 * It listens for the formora:submit event and logs the data.
 *
 * INSTALLATION:
 * Add this script to your page as an inline script or loaded JS file.
 *
 * USAGE:
 * The adapter will automatically listen for form submission events.
 * Implement your own logic to inject e.detail.raw into your chat input.
 */

window.addEventListener('formora:submit', (e) => {
  // e.detail.raw  → the __formora__... string to send as chat message
  // e.detail.parsed → the structured data object

  console.log('Formora form submitted:', e.detail.parsed);

  // TODO: Inject e.detail.raw into your chat input and trigger send
  // Example:
  // const chatInput = document.querySelector('#your-chat-input');
  // if (chatInput) {
  //   chatInput.value = e.detail.raw;
  //   chatInput.dispatchEvent(new Event('input'));
  //   // Trigger send - varies by framework
  // }
});

/**
 * TESTED VERSION: N/A (generic adapter)
 */
