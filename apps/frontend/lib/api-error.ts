const NETWORK_ERROR_CODES = new Set([
    'ECONNREFUSED',
    'ECONNRESET',
    'ENETUNREACH',
    'ENOTFOUND',
    'ETIMEDOUT',
    'UND_ERR_CONNECT_TIMEOUT',
]);

function normalizeMessage(message: string, fallback: string) {
    const trimmed = message.trim();
    if (!trimmed) return fallback;

    return trimmed;
}

function getErrorCode(error: unknown): string | undefined {
    if (!error || typeof error !== 'object') return undefined;

    if ('code' in error && typeof (error as { code?: unknown }).code === 'string') {
        return (error as { code: string }).code;
    }

    if (
        'cause' in error &&
        error.cause &&
        typeof error.cause === 'object' &&
        'code' in error.cause &&
        typeof (error.cause as { code?: unknown }).code === 'string'
    ) {
        return (error.cause as { code: string }).code;
    }

    return undefined;
}

export function getApiErrorMessage(error: unknown, fallback: string) {
    const errorCode = getErrorCode(error);
    if (errorCode && NETWORK_ERROR_CODES.has(errorCode)) {
        return 'Could not reach the server. Try again.';
    }

    if (typeof error === 'string') {
        if (error.toLowerCase().includes('too many')) {
            return 'Too many requests. Wait a moment and try again.';
        }
        return normalizeMessage(error, fallback);
    }

    if (error instanceof Error) {
        if (error.message.toLowerCase().includes('too many')) {
            return 'Too many requests. Wait a moment and try again.';
        }
        return normalizeMessage(error.message, fallback);
    }

    if (error && typeof error === 'object' && 'error' in error) {
        const message = (error as { error?: unknown }).error;
        if (typeof message === 'string') {
            if (message.toLowerCase().includes('too many')) {
                return 'Too many requests. Wait a moment and try again.';
            }
            return normalizeMessage(message, fallback);
        }
    }

    return fallback;
}
