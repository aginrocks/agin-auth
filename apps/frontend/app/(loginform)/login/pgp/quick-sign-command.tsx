import { useState } from 'react';
import { IconTerminal2, IconChevronDown } from '@tabler/icons-react';
import { AnimatePresence, motion } from 'motion/react';
import { CopyButton } from '@components/copy-button';

interface QuickSignCommandProps {
    gpgCommand: string;
}

export function QuickSignCommand({ gpgCommand }: QuickSignCommandProps) {
    const [showCommand, setShowCommand] = useState(false);

    return (
        <div>
            <button
                type="button"
                onClick={() => setShowCommand(!showCommand)}
                className="flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground transition-colors"
            >
                <IconTerminal2 className="size-3.5" />
                <span>Quick sign command</span>
                <motion.span
                    animate={{ rotate: showCommand ? 180 : 0 }}
                    transition={{ duration: 0.2 }}
                    className="inline-flex"
                >
                    <IconChevronDown className="size-3" />
                </motion.span>
            </button>
            <AnimatePresence initial={false}>
                {showCommand && (
                    <motion.div
                        initial={{ height: 0, opacity: 0 }}
                        animate={{ height: 'auto', opacity: 1 }}
                        exit={{ height: 0, opacity: 0 }}
                        transition={{ duration: 0.2, ease: 'easeInOut' }}
                        className="overflow-hidden"
                    >
                        <div className="mt-1.5 rounded-md border border-input dark:bg-input/30 bg-transparent px-3 py-2.5 flex items-start justify-between gap-2">
                            <pre className="font-mono text-[11px] text-muted-foreground whitespace-pre-wrap break-all select-all flex-1">
                                {gpgCommand}
                            </pre>
                            <CopyButton text={gpgCommand} compact />
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>
        </div>
    );
}
