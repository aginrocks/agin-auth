'use client';

import { useState } from 'react';
import { $api } from '@lib/providers/api';
import { useWebAuthnRegistration } from '@lib/hooks';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { Label } from '@components/ui/label';
import { IconFingerprint, IconKey, IconTrash } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { ErrorMsg, ExpandForm } from './helpers';

export function WebAuthnRow({ keys, onRefetch }: { keys: { display_name: string }[]; onRefetch: () => void }) {
    const [adding, setAdding] = useState(false);
    const [newName, setNewName] = useState('');
    const [error, setError] = useState('');
    const [deletingKey, setDeletingKey] = useState<string | null>(null);
    const [confirmingDelete, setConfirmingDelete] = useState<string | null>(null);

    const webAuthn = useWebAuthnRegistration();
    const deleteKey = $api.useMutation('delete', '/api/settings/factors/webauthn/delete/{display_name}');

    const handleAdd = async (e: React.FormEvent) => {
        e.preventDefault(); setError('');
        try { await webAuthn.registerAsync(newName); setNewName(''); setAdding(false); onRefetch(); }
        catch { setError('Failed to register passkey.'); }
    };

    const handleDelete = async (name: string) => {
        setDeletingKey(name);
        setConfirmingDelete(null);
        try { await deleteKey.mutateAsync({ params: { path: { display_name: name } } }); onRefetch(); }
        catch { setError('Failed to delete.'); }
        finally { setDeletingKey(null); }
    };

    return (
        <FactorRow icon={<IconFingerprint />} name="Passkeys" description="Phishing-resistant authentication using your device or a hardware security key." tag={{ label: keys.length > 0 ? `${keys.length} key${keys.length > 1 ? 's' : ''}` : 'Disabled', enabled: keys.length > 0 }} onToggle={() => setAdding(v => !v)} open={adding}>
            <div className="ml-9 px-5">
                {adding && keys.length > 0 && (
                    <div className="space-y-1 mt-2 mb-3 max-w-sm">
                        {keys.map(key => (
                            <div key={key.display_name} className="flex items-center justify-between rounded-lg border border-border px-3 py-2 bg-muted/30">
                                <div className="flex items-center gap-2">
                                    <IconKey size={12} className="text-muted-foreground" />
                                    <span className="text-sm">{key.display_name}</span>
                                </div>
                                {confirmingDelete === key.display_name ? (
                                    <div className="flex items-center gap-1.5">
                                        <button onClick={() => handleDelete(key.display_name)} disabled={deletingKey === key.display_name}
                                            className="text-xs text-destructive hover:text-destructive/80 transition-colors disabled:opacity-30 font-medium">
                                            {deletingKey === key.display_name ? 'Deleting…' : 'Confirm'}
                                        </button>
                                        <button onClick={() => setConfirmingDelete(null)}
                                            className="text-xs text-muted-foreground hover:text-foreground transition-colors">
                                            Cancel
                                        </button>
                                    </div>
                                ) : (
                                    <button onClick={() => setConfirmingDelete(key.display_name)} disabled={deletingKey === key.display_name}
                                        aria-label={`Delete ${key.display_name}`}
                                        className="text-muted-foreground hover:text-destructive transition-colors disabled:opacity-30">
                                        <IconTrash size={13} />
                                    </button>
                                )}
                            </div>
                        ))}
                    </div>
                )}
                <ErrorMsg msg={error} />
                <ExpandForm open={adding}>
                    <form onSubmit={handleAdd} className="space-y-3 max-w-sm pb-4">
                        <div className="space-y-1.5">
                            <Label htmlFor="webauthn-name" className="text-xs">Key name</Label>
                            <Input id="webauthn-name" value={newName} onChange={e => setNewName(e.target.value)}
                                placeholder="YubiKey 5, iPhone Face ID…" className="h-9 text-sm" required maxLength={32} />
                        </div>
                        <div className="flex gap-2">
                            <Button size="sm" type="submit">Register</Button>
                            <Button size="sm" variant="ghost" type="button" onClick={() => { setAdding(false); setError(''); }}>Cancel</Button>
                        </div>
                    </form>
                </ExpandForm>
            </div>
        </FactorRow>
    );
}
