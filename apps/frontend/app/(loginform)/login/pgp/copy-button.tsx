import { IconCheck, IconCopy } from '@tabler/icons-react';

export function CopyButton({ onCopy, copied }: { onCopy: () => void; copied: boolean }) {
    return (
        <button
            type="button"
            onClick={onCopy}
            className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors w-[52px]"
        >
            {copied ? <IconCheck className="size-3" /> : <IconCopy className="size-3" />}
            {copied ? 'Copied' : 'Copy'}
        </button>
    );
}
