'use client';

import { useState } from 'react';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { Label } from '@components/ui/label';
import { IconDeviceMobile, IconCheck } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { CopyButton, ErrorMsg, ExpandForm } from './helpers';

export function TotpRow({ totp, onRefetch }: { totp: { display_name: string; fully_enabled: boolean } | null | undefined; onRefetch: () => void }) {
    const [step, setStep] = useState<'idle' | 'name' | 'confirm'>('idle');
    const [setupData, setSetupData] = useState<{ secret: string; qr: string } | null>(null);
    const [displayName, setDisplayName] = useState('');
    const [code, setCode] = useState('');
    const [error, setError] = useState('');

    const enable = $api.useMutation('post', '/api/settings/factors/totp/enable');
    const confirm = $api.useMutation('post', '/api/settings/factors/totp/enable/confirm');
    const disable = $api.useMutation('delete', '/api/settings/factors/totp/disable');

    const isEnabled = totp?.fully_enabled ?? false;

    const startSetup = async (e: React.FormEvent) => {
        e.preventDefault(); setError('');
        try { const d = await enable.mutateAsync({ body: { display_name: displayName } }); setSetupData(d); setStep('confirm'); }
        catch { setError('Failed to start setup.'); }
    };

    const confirmSetup = async (e: React.FormEvent) => {
        e.preventDefault(); setError('');
        try { await confirm.mutateAsync({ body: { code } }); setStep('idle'); setSetupData(null); setCode(''); setDisplayName(''); onRefetch(); }
        catch { setError('Invalid code, try again.'); }
    };

    const handleDisable = async () => {
        setError('');
        try { await disable.mutateAsync({}); onRefetch(); }
        catch { setError('Failed to disable.'); }
    };

    const handleToggle = () => {
        if (step !== 'idle') { setStep('idle'); setSetupData(null); setError(''); }
        else if (isEnabled) { setStep('idle'); }
        else { setStep('name'); }
    };

    return (
        <FactorRow icon={<IconDeviceMobile />} name="Authenticator App" description="Time-based one-time passwords from your authenticator app." tag={{ label: isEnabled ? totp?.display_name ?? 'Enabled' : 'Disabled', enabled: isEnabled }} onToggle={handleToggle} open={step !== 'idle'}>
            <div className="ml-9 px-5">
                {step === 'idle' && isEnabled && (
                    <>
                        <ErrorMsg msg={error} />
                        <button onClick={handleDisable} disabled={disable.isPending}
                            className="mt-2 pb-3 flex items-center gap-1 text-xs text-destructive/60 hover:text-destructive transition-colors disabled:opacity-50">
                            {disable.isPending ? 'Disabling…' : 'Disable'}
                        </button>
                    </>
                )}

                <ExpandForm open={step === 'name'}>
                    <form onSubmit={startSetup} className="space-y-3 max-w-sm pb-4">
                        <div className="space-y-1.5">
                            <Label htmlFor="totp-name" className="text-xs">Authenticator name</Label>
                            <Input id="totp-name" value={displayName} onChange={e => setDisplayName(e.target.value)}
                                placeholder="Authy, Google Authenticator…" className="h-9 text-sm" required maxLength={32} />
                        </div>
                        <ErrorMsg msg={error} />
                        <div className="flex gap-2">
                            <Button size="sm" type="submit" disabled={enable.isPending}>
                                {enable.isPending ? 'Generating…' : 'Continue'}
                            </Button>
                            <Button size="sm" variant="ghost" type="button" onClick={() => { setStep('idle'); setError(''); }}>Cancel</Button>
                        </div>
                    </form>
                </ExpandForm>

                <ExpandForm open={step === 'confirm' && !!setupData}>
                    {setupData && (
                        <div className="space-y-3 max-w-sm pb-4">
                            <div className="rounded-lg border border-border bg-muted/40 p-3 space-y-2.5">
                                <p className="text-xs text-muted-foreground">Add to your authenticator app, then enter the code below.</p>
                                <div>
                                    <div className="flex items-center justify-between mb-1">
                                        <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">Secret</span>
                                        <CopyButton text={setupData.secret} />
                                    </div>
                                    <code className="font-mono text-xs break-all text-foreground">{setupData.secret}</code>
                                </div>
                                <div>
                                    <div className="flex items-center justify-between mb-1">
                                        <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">OTP URL</span>
                                        <CopyButton text={setupData.qr} />
                                    </div>
                                    <code className="font-mono text-[10px] break-all text-muted-foreground">{setupData.qr}</code>
                                </div>
                            </div>
                            <form onSubmit={confirmSetup} className="space-y-3">
                                <div className="space-y-1.5">
                                    <Label htmlFor="totp-code" className="text-xs">Verification code</Label>
                                    <Input id="totp-code" value={code} onChange={e => setCode(e.target.value.replace(/\D/g, '').slice(0, 6))}
                                        placeholder="000000" className="h-9 font-mono tracking-[0.3em] text-sm text-center" maxLength={6} required />
                                </div>
                                <ErrorMsg msg={error} />
                                <div className="flex gap-2">
                                    <Button size="sm" type="submit" disabled={confirm.isPending}>
                                        <IconCheck size={13} /> {confirm.isPending ? 'Verifying…' : 'Confirm'}
                                    </Button>
                                    <Button size="sm" variant="ghost" type="button" onClick={() => { setStep('idle'); setSetupData(null); setError(''); }}>Cancel</Button>
                                </div>
                            </form>
                        </div>
                    )}
                </ExpandForm>
            </div>
        </FactorRow>
    );
}
