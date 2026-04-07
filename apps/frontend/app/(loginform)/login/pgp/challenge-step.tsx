import { IconRefresh } from '@tabler/icons-react';
import { motion } from 'motion/react';
import { useClipboard } from '@mantine/hooks';
import { CopyButton } from './copy-button';

interface ChallengeStepProps {
    challenge: string;
    refreshChallenge: () => void;
    isPending: boolean;
    refreshSpin: number;
}

export function ChallengeStep({ challenge, refreshChallenge, isPending, refreshSpin }: ChallengeStepProps) {
    const clipboard = useClipboard({ timeout: 1500 });

    return (
        <div className="space-y-1.5">
            <div className="flex items-center justify-between">
                <label className="text-sm font-medium">
                    <span className="text-muted-foreground mr-1.5">1.</span>
                    Copy the challenge
                </label>
                <div className="flex items-center gap-2">
                    <CopyButton onCopy={() => clipboard.copy(challenge)} copied={clipboard.copied} />
                    <button
                        type="button"
                        onClick={refreshChallenge}
                        disabled={isPending}
                        className="text-muted-foreground hover:text-foreground transition-colors disabled:opacity-50"
                    >
                        <motion.span
                            key={refreshSpin}
                            animate={{ rotate: -360 }}
                            transition={{ type: 'spring', stiffness: 200, damping: 15 }}
                            className="inline-flex"
                        >
                            <IconRefresh className="size-3.5" />
                        </motion.span>
                    </button>
                </div>
            </div>
            <div className="rounded-md border border-input dark:bg-input/30 bg-transparent px-3 py-2.5 font-mono text-xs break-all select-all cursor-text">
                {challenge || <span className="text-muted-foreground">Generating...</span>}
            </div>
        </div>
    );
}
