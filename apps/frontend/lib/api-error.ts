function normalizeMessage(message: string, fallback: string) {
    const trimmed = message.trim();
    if (!trimmed) return fallback;

    if (/failed to fetch|fetch failed|networkerror|load failed/i.test(trimmed)) {
        return 'Could not reach the server. Try again.';
    }

    return trimmed;
}

export function getApiErrorMessage(error: unknown, fallback: string) {
    if (typeof error === 'string') {
        return normalizeMessage(error, fallback);
    }

    if (error instanceof Error) {
        return normalizeMessage(error.message, fallback);
    }

    if (error && typeof error === 'object' && 'error' in error) {
        const message = (error as { error?: unknown }).error;
        if (typeof message === 'string') {
            return normalizeMessage(message, fallback);
        }
    }

    return fallback;
}
