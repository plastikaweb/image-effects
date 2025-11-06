async function init() {
    const input = document.getElementById('upload');
    const reader = new FileReader();

    let rustApp = null;

    try {
        rustApp = await import('../pkg');
    } catch(e) {
        console.error('Error loading WASM module:', e);
        return;
    }

    console.log(rustApp);

    reader.onloadend = () => {
        const base64 = reader.result.replace(/^data:image\/(png|jpg|jpeg);base64,/, '');
        const imageDataUrl = rustApp.grayscale(base64);

        const outputImage = document.getElementById('new-img');
        outputImage.src = imageDataUrl;
    }

    input.addEventListener('change', () => {
        reader.readAsDataURL(input.files[0]);

    });
}

init();
