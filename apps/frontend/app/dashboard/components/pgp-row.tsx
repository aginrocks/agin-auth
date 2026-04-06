'use client';

import { useState, useCallback } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { Label } from '@components/ui/label';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { Input } from '@components/ui/input';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '@components/ui/dialog';
import { IconKey, IconPlus } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { ExpandForm } from './helpers';
import { FactorKeyItem } from './factor-key-item';

const pgpSchema = z.object({
    display_name: z.string().min(1, 'Required').max(32),
    public_key: z.string().min(1, 'Required'),
});
type PgpForm = z.infer<typeof pgpSchema>;

export function PgpRow({ pgp, onRefetch }: { pgp: { fingerprint: string; display_name: string }[]; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [addOpen, setAddOpen] = useState(false);
    const [deleteTarget, setDeleteTarget] = useState<{ fingerprint: string; display_name: string } | null>(null);
    const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);

    const form = useForm<PgpForm>({
        resolver: zodResolver(pgpSchema),
        defaultValues: { display_name: '', public_key: '' },
    });

    const enable = $api.useMutation('post', '/api/settings/factors/pgp/enable', {
        onSuccess: () => {
            form.reset();
            setAddOpen(false);
            onRefetch();
        },
        onError: () => {
            form.setError('public_key', { message: 'Invalid key or failed to add.' });
        },
    });

    const deleteKey = $api.useMutation('delete', '/api/settings/factors/pgp/delete/{fingerprint}', {
        onSuccess: () => { setDeleteDialogOpen(false); onRefetch(); },
    });

    const isEnabled = pgp.length > 0;

    const onSubmit = useCallback((data: PgpForm) => {
        enable.mutate({ body: data });
    }, [enable]);

    const handleToggle = useCallback(() => {
        setOpen(v => !v);
    }, []);

    const handleDelete = () => {
        if (!deleteTarget) return;
        deleteKey.mutate({ params: { path: { fingerprint: deleteTarget.fingerprint } } });
    };

    return (
        <>
            <FactorRow
                icon={<IconKey />}
                name="PGP Key"
                description="Authenticate by signing a server challenge with your PGP private key."
                tag={{ label: isEnabled ? `${pgp.length} key${pgp.length > 1 ? 's' : ''}` : 'Disabled', enabled: isEnabled }}
                onToggle={handleToggle}
                open={open}
            >
                <ExpandForm open={open}>
                    <div className="ml-9 px-5 pb-3">
                        {isEnabled && (
                            <div className="space-y-1 mb-3 max-w-sm">
                                {pgp.map(key => (
                                    <FactorKeyItem
                                        key={key.fingerprint}
                                        icon={<IconKey size={14} className="text-muted-foreground" />}
                                        name={key.display_name}
                                        subtitle={key.fingerprint}
                                        onRemove={() => { setDeleteTarget(key); setDeleteDialogOpen(true); }}
                                    />
                                ))}
                            </div>
                        )}
                        <Button size="sm" onClick={() => { setAddOpen(true); form.reset(); }}>
                            <IconPlus size={14} /> Add PGP key
                        </Button>
                    </div>
                </ExpandForm>
            </FactorRow>

            {/* Add PGP key modal */}
            <Dialog open={addOpen} onOpenChange={(v) => { if (!v) form.reset(); setAddOpen(v); }}>
                <DialogContent className="sm:max-w-md">
                    <DialogHeader>
                        <DialogTitle>Add PGP key</DialogTitle>
                        <DialogDescription>
                            Provide a name and your ASCII-armored public key.
                        </DialogDescription>
                    </DialogHeader>
                    <Form {...form}>
                        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-3">
                            <FormField control={form.control} name="display_name" render={({ field }) => (
                                <FormItem className="space-y-1.5">
                                    <Label className="text-xs">Name</Label>
                                    <FormControl>
                                        <Input {...field} placeholder="Work key" className="h-9 text-sm" maxLength={32} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )} />
                            <FormField control={form.control} name="public_key" render={({ field }) => (
                                <FormItem className="space-y-1.5">
                                    <Label className="text-xs">Public key (ASCII-armored)</Label>
                                    <FormControl>
                                        <textarea {...field}
                                            placeholder="-----BEGIN PGP PUBLIC KEY BLOCK-----"
                                            className="w-full rounded-md border border-input bg-background px-3 py-2 font-mono text-[11px] placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring/50 min-h-[80px] resize-none" />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )} />
                            <div className="flex gap-2 justify-end">
                                <Button variant="outline" type="button" onClick={() => { setAddOpen(false); form.reset(); }}>Cancel</Button>
                                <Button type="submit" disabled={enable.isPending}>
                                    {enable.isPending ? 'Adding…' : 'Add key'}
                                </Button>
                            </div>
                        </form>
                    </Form>
                </DialogContent>
            </Dialog>

            {/* Delete PGP key confirmation */}
            <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Remove PGP key</DialogTitle>
                        <DialogDescription>
                            Are you sure you want to delete{' '}<span className="font-medium text-foreground">{deleteTarget?.display_name}</span>? This action cannot be undone.
                        </DialogDescription>
                    </DialogHeader>
                    {deleteKey.isError && (
                        <p className="text-xs text-destructive">Failed to remove PGP key.</p>
                    )}
                    <DialogFooter>
                        <Button variant="outline" onClick={() => setDeleteDialogOpen(false)} disabled={deleteKey.isPending}>
                            Cancel
                        </Button>
                        <Button variant="destructive" onClick={handleDelete} disabled={deleteKey.isPending}>
                            {deleteKey.isPending ? 'Removing…' : 'Remove'}
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </>
    );
}
