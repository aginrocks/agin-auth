'use client';

import { useState } from 'react';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@components/ui/dialog';
import { IconLifebuoy, IconCheck } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { CopyButton, ErrorMsg, ExpandForm } from './helpers';

export function RecoveryCodesRow({ remaining, onRefetch }: { remaining: number; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [codesModalOpen, setCodesModalOpen] = useState(false);
    const [codes, setCodes] = useState<string[] | null>(null);
    const [error, setError] = useState('');

    const enable = $api.useMutation('post', '/api/settings/factors/recovery-codes/enable', {
        onSuccess: (data) => { setCodes(data.codes); setCodesModalOpen(true); onRefetch(); },
        onError: () => { setError('Failed to generate codes.'); },
    });
    const reset = $api.useMutation('post', '/api/settings/factors/recovery-codes/reset', {
        onSuccess: (data) => { setCodes(data.codes); setCodesModalOpen(true); onRefetch(); },
        onError: () => { setError('Failed to regenerate.'); },
    });

    const isEnabled = remaining > 0;

    const handleEnable = () => { setError(''); enable.mutate({}); };
    const handleReset = () => { setError(''); reset.mutate({}); };

    const handleCloseModal = () => {
        setCodesModalOpen(false);
        setCodes(null);
    };

    return (
        <>
            <FactorRow icon={<IconLifebuoy />} name="Recovery Codes" description="One-time emergency codes to regain access if you lose your other factors." tag={{ label: isEnabled ? `${remaining} remaining` : 'Disabled', enabled: isEnabled }} onToggle={() => { setOpen(v => !v); setError(''); }} open={open} last>
                <ExpandForm open={open}>
                    <div className="ml-9 px-5 pb-4 max-w-md">
                        <div className="space-y-3">
                            <p className="text-xs text-muted-foreground leading-relaxed">
                                {isEnabled
                                    ? `${remaining} code${remaining !== 1 ? 's' : ''} remaining. Regenerating will replace all current codes.`
                                    : 'Generate one-time codes to access your account if you lose your other sign-in methods.'}
                            </p>
                            <ErrorMsg msg={error} />
                            <div className="flex gap-2">
                                <Button size="sm" onClick={isEnabled ? handleReset : handleEnable} disabled={enable.isPending || reset.isPending}>
                                    {enable.isPending || reset.isPending ? 'Generating…' : isEnabled ? 'Regenerate' : 'Generate'}
                                </Button>
                                <Button size="sm" variant="ghost" onClick={() => { setOpen(false); setError(''); }}>Cancel</Button>
                            </div>
                        </div>
                    </div>
                </ExpandForm>
            </FactorRow>

            {/* Recovery codes modal */}
            <Dialog open={codesModalOpen} onOpenChange={(v) => { if (!v) handleCloseModal(); }}>
                <DialogContent className="sm:max-w-md">
                    <DialogHeader>
                        <DialogTitle>Your recovery codes</DialogTitle>
                        <DialogDescription>
                            Store these codes in a safe place. Each code can only be used once.
                        </DialogDescription>
                    </DialogHeader>
                    {codes && (
                        <div className="space-y-4">
                            <div className="rounded-xl border border-border bg-card p-4">
                                <div className="flex items-center justify-between mb-3">
                                    <p className="text-xs font-medium text-foreground">Recovery codes</p>
                                    <CopyButton text={codes.join('\n')} />
                                </div>
                                <div className="grid grid-cols-2 gap-x-8 gap-y-2 px-1">
                                    {codes.map((c, i) => (
                                        <div key={c} className="flex items-center gap-2.5">
                                            <span className="text-[10px] tabular-nums text-muted-foreground/60 w-3 text-right">{i + 1}</span>
                                            <code className="font-mono text-[13px] tracking-wide text-foreground">{c}</code>
                                        </div>
                                    ))}
                                </div>
                            </div>
                            <div className="flex justify-end">
                                <Button onClick={handleCloseModal}>
                                    <IconCheck size={14} /> Done, I saved them
                                </Button>
                            </div>
                        </div>
                    )}
                </DialogContent>
            </Dialog>
        </>
    );
}
