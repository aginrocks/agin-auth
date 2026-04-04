'use client';

import { useState } from 'react';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { Label } from '@components/ui/label';
import { IconKey } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { ErrorMsg, ExpandForm } from './helpers';

export function PgpRow({ pgp, onRefetch }: { pgp: { fingerprint: string; display_name: string }[]; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [displayName, setDisplayName] = useState('');
    const [publicKey, setPublicKey] = useState('');
    const [error, setError] = useState('');

    const enable = $api.useMutation('post', '/api/settings/factors/pgp/enable');
    const disable = $api.useMutation('delete', '/api/settings/factors/pgp/disable');

    const isEnabled = pgp.length > 0;

    const handleEnable = async (e: React.FormEvent) => {
        e.preventDefault(); setError('');
        try { await enable.mutateAsync({ body: { display_name: displayName, public_key: publicKey } }); setOpen(false); setDisplayName(''); setPublicKey(''); onRefetch(); }
        catch { setError('Invalid key or failed to add.'); }
    };

    const handleDisable = async () => {
        setError('');
        try { await disable.mutateAsync({}); onRefetch(); }
        catch { setError('Failed to remove.'); }
    };

    return (
        <FactorRow icon={<IconKey />} name="PGP Key" description="Authenticate by signing a server challenge with your PGP private key." tag={{ label: isEnabled ? pgp[0]?.display_name ?? 'Enabled' : 'Disabled', enabled: isEnabled }} onToggle={() => { if (!isEnabled) setOpen(v => !v); }} open={open}>
            <div className="ml-9 px-5">
                {isEnabled ? (
                    <div className="space-y-2 mt-2 pb-3">
                        <div className="rounded-lg border border-border bg-muted/30 px-3 py-2 max-w-sm">
                            <p className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground mb-1">Fingerprint</p>
                            <code className="font-mono text-[11px] text-foreground break-all">{pgp[0]?.fingerprint}</code>
                        </div>
                        <ErrorMsg msg={error} />
                        <button onClick={handleDisable} disabled={disable.isPending}
                            className="flex items-center gap-1 text-xs text-destructive/60 hover:text-destructive transition-colors disabled:opacity-50">
                            {disable.isPending ? 'Removing…' : 'Remove key'}
                        </button>
                    </div>
                ) : (
                    <ExpandForm open={open}>
                        <form onSubmit={handleEnable} className="space-y-3 max-w-sm pb-4">
                            <div className="space-y-1.5">
                                <Label htmlFor="pgp-name" className="text-xs">Name</Label>
                                <Input id="pgp-name" value={displayName} onChange={e => setDisplayName(e.target.value)}
                                    placeholder="Work key" className="h-9 text-sm" required maxLength={32} />
                            </div>
                            <div className="space-y-1.5">
                                <Label className="text-xs">Public key (ASCII-armored)</Label>
                                <textarea value={publicKey} onChange={e => setPublicKey(e.target.value)}
                                    placeholder="-----BEGIN PGP PUBLIC KEY BLOCK-----"
                                    className="w-full rounded-md border border-input bg-background px-3 py-2 font-mono text-[11px] placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring/50 min-h-[80px] resize-none"
                                    required />
                            </div>
                            <ErrorMsg msg={error} />
                            <div className="flex gap-2">
                                <Button size="sm" type="submit" disabled={enable.isPending}>
                                    {enable.isPending ? 'Adding…' : 'Add key'}
                                </Button>
                                <Button size="sm" variant="ghost" type="button" onClick={() => { setOpen(false); setError(''); }}>Cancel</Button>
                            </div>
                        </form>
                    </ExpandForm>
                )}
            </div>
        </FactorRow>
    );
}
