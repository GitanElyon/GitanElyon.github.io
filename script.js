document.addEventListener("DOMContentLoaded", function() {
    const iframes = document.querySelectorAll("iframe");
    iframes.forEach(iframe => {
        iframe.onload = function() {
            const iframeDocument = iframe.contentDocument || iframe.contentWindow.document;
            const style = document.createElement("style");
            style.textContent = `
                body {
                    font-family: Arial, sans-serif;
                    background-color: #171717;
                    color: #ffffff;
                }
                * {
                    font-family: inherit;
                    color: inherit;
                }
            `;
            iframeDocument.head.appendChild(style);
        };
    });
});