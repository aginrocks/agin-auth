'use client';

import { IconCheck, IconCopy } from '@tabler/icons-react';
import { useClipboard } from '@mantine/hooks';
import { Button } from '@components/ui/button';

export function CopyButton({ text, compact }: { text: string; compact?: boolean }) {
    const clipboard = useClipboard({ timeout: 1500 });

    if (compact) {
        return (
            <button
                type="button"
                onClick={() => clipboard.copy(text)}
                className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors w-[52px]"
            >
                {clipboard.copied ? <IconCheck className="size-3" /> : <IconCopy className="size-3" />}
                {clipboard.copied ? 'Copied' : 'Copy'}
            </button>
        );
    }

    return (
        <Button
            type="button"
            variant="ghost"
            size="sm"
            className="h-auto px-2 py-1 text-xs text-muted-foreground"
            onClick={() => clipboard.copy(text)}
        >
            {clipboard.copied ? <IconCheck size={13} /> : <IconCopy size={13} />}
            {clipboard.copied ? 'Copied' : 'Copy'}
        </Button>
    );
}
