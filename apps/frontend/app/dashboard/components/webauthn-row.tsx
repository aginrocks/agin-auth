'use client';

import { useEffect, useRef, useState } from 'react';
import { $api } from '@lib/providers/api';
import { useWebAuthnRegistration } from '@lib/hooks';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { Label } from '@components/ui/label';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '@components/ui/dialog';
import { IconFingerprint, IconPlus } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { ErrorMsg, ExpandForm } from './helpers';
import { FactorKeyItem } from './factor-key-item';

export function WebAuthnRow({ keys, onRefetch }: { keys: { credential_id: string; display_name: string }[]; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [addOpen, setAddOpen] = useState(false);
    const [newName, setNewName] = useState('');
    const [error, setError] = useState('');
    const [deleteTarget, setDeleteTarget] = useState<{ credential_id: string; display_name: string } | null>(null);
    const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
    const addDialogResetTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    const webAuthn = useWebAuthnRegistration();
    const deleteKey = $api.useMutation('delete', '/api/settings/factors/webauthn/delete/{credential_id}', {
        onSuccess: () => { setDeleteDialogOpen(false); onRefetch(); },
        onError: () => setError('Failed to delete passkey.'),
    });

    const handleAdd = async (e: React.FormEvent) => {
        e.preventDefault(); setError('');
        try {
            await webAuthn.registerAsync(newName);
            closeAddDialog();
            onRefetch();
        } catch {
            setError('Failed to register passkey.');
        }
    };

    const handleDelete = () => {
        if (!deleteTarget) return;
        deleteKey.mutate({ params: { path: { credential_id: deleteTarget.credential_id } } });
    };

    const closeAddDialog = () => {
        if (addDialogResetTimeoutRef.current) {
            clearTimeout(addDialogResetTimeoutRef.current);
        }

        setAddOpen(false);
        addDialogResetTimeoutRef.current = setTimeout(() => {
            setNewName('');
            setError('');
            addDialogResetTimeoutRef.current = null;
        }, 200);
    };

    useEffect(() => {
        return () => {
            if (addDialogResetTimeoutRef.current) {
                clearTimeout(addDialogResetTimeoutRef.current);
            }
        };
    }, []);

    return (
        <>
            <FactorRow icon={<IconFingerprint />} name="Passkeys" description="Phishing-resistant authentication using your device or a hardware security key." tag={{ label: keys.length > 0 ? `${keys.length} key${keys.length > 1 ? 's' : ''}` : 'Disabled', enabled: keys.length > 0 }} onToggle={() => setOpen(v => !v)} open={open}>
                <ExpandForm open={open}>
                    <div className="ml-9 px-5 pb-3">
                        {keys.length > 0 && (
                            <div className="space-y-1 mb-3 max-w-sm">
                                {keys.map(key => (
                                    <FactorKeyItem
                                        key={key.credential_id}
                                        icon={<IconFingerprint size={14} className="text-muted-foreground" />}
                                        name={key.display_name}
                                        subtitle="Passkey"
                                        onRemove={() => { setDeleteTarget(key); setDeleteDialogOpen(true); }}
                                    />
                                ))}
                            </div>
                        )}
                        <ErrorMsg msg={error} />
                        <Button size="sm" onClick={() => {
                            if (addDialogResetTimeoutRef.current) {
                                clearTimeout(addDialogResetTimeoutRef.current);
                                addDialogResetTimeoutRef.current = null;
                            }
                            setNewName('');
                            setError('');
                            setAddOpen(true);
                        }}>
                            <IconPlus size={14} /> Add passkey
                        </Button>
                    </div>
                </ExpandForm>
            </FactorRow>

            {/* Add passkey modal */}
            <Dialog
                open={addOpen}
                onOpenChange={(v) => {
                    if (v) {
                        if (addDialogResetTimeoutRef.current) {
                            clearTimeout(addDialogResetTimeoutRef.current);
                            addDialogResetTimeoutRef.current = null;
                        }
                        setAddOpen(true);
                        return;
                    }

                    closeAddDialog();
                }}
            >
                <DialogContent className="sm:max-w-md">
                    <DialogHeader>
                        <DialogTitle>Add passkey</DialogTitle>
                        <DialogDescription>
                            Give your passkey a name, then follow your browser's prompt to register it.
                        </DialogDescription>
                    </DialogHeader>
                    <form onSubmit={handleAdd} className="space-y-3">
                        <div className="space-y-1.5">
                            <Label htmlFor="webauthn-name" className="text-xs">Key name</Label>
                            <Input id="webauthn-name" value={newName} onChange={e => setNewName(e.target.value)}
                                placeholder="YubiKey 5, iPhone Face ID…" className="h-9 text-sm" required maxLength={32} />
                        </div>
                        <ErrorMsg msg={error} />
                        <div className="flex gap-2 justify-end">
                            <Button variant="outline" type="button" onClick={closeAddDialog}>Cancel</Button>
                            <Button type="submit">Register</Button>
                        </div>
                    </form>
                </DialogContent>
            </Dialog>

            {/* Delete passkey confirmation */}
            <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Delete passkey</DialogTitle>
                        <DialogDescription>
                            Are you sure you want to delete{' '}<span className="font-medium text-foreground">{deleteTarget?.display_name}</span>? This action cannot be undone.
                        </DialogDescription>
                    </DialogHeader>
                    <DialogFooter>
                        <Button variant="outline" onClick={() => setDeleteDialogOpen(false)} disabled={deleteKey.isPending}>
                            Cancel
                        </Button>
                        <Button variant="destructive" onClick={handleDelete} disabled={deleteKey.isPending}>
                            {deleteKey.isPending ? 'Deleting…' : 'Delete'}
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </>
    );
}
