function getProvider() {
    if ("phantom" in window) {
        const provider = window.phantom?.solana;

        if (provider?.isPhantom) {
            return provider;
        }
    }
}

export async function connect() {
    const provider = getProvider();
    try {
        const response = await provider.connect();
        return response.publicKey.toString();
    } catch (_) {
        return "";
    }
}

export async function trustedConnect() {
    const provider = getProvider();
    try {
        const response = await provider.connect({onlyIfTrusted: true});
        return response.publicKey.toString();
    } catch (_) {
        return "";
    }
}
