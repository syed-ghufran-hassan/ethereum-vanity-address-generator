document.getElementById("generateButton").addEventListener("click", async () => {
    const pattern = document.getElementById("vanityPattern").value;
    const resultDiv = document.getElementById("result");

    try {
        // Invoke the Tauri command to generate the Ethereum address
        const address = await window.__TAURI__.invoke("generate_ethereum_address", { pattern });
        resultDiv.innerText = `Generated Address: ${address}`;
    } catch (error) {
        console.error("Error generating address:", error);
        resultDiv.innerText = "Failed to generate address. Check console for details.";
    }
});
