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

    document.getElementById('header-arrow').addEventListener('click', function() {
        document.getElementById('projects-section').scrollIntoView({ behavior: 'smooth' });
    });

    document.getElementById('projects-arrow').addEventListener('click', function() {
        document.getElementById('skills-section').scrollIntoView({ behavior: 'smooth' });
    });

    document.getElementById('skills-arrow').addEventListener('click', function() {
        document.getElementById('about-section').scrollIntoView({ behavior: 'smooth' });
    });

});