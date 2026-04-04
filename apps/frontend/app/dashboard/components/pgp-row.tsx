'use client';

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { Label } from '@components/ui/label';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { Input } from '@components/ui/input';
import { IconKey, IconTrash } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { ErrorMsg, ExpandForm } from './helpers';

const pgpSchema = z.object({
    display_name: z.string().min(1, 'Required').max(32),
    public_key: z.string().min(1, 'Required'),
});
type PgpForm = z.infer<typeof pgpSchema>;

export function PgpRow({ pgp, onRefetch }: { pgp: { fingerprint: string; display_name: string }[]; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [confirmDelete, setConfirmDelete] = useState(false);
    const [deleteError, setDeleteError] = useState('');

    const enable = $api.useMutation('post', '/api/settings/factors/pgp/enable');
    const disable = $api.useMutation('delete', '/api/settings/factors/pgp/disable');

    const isEnabled = pgp.length > 0;

    const form = useForm<PgpForm>({
        resolver: zodResolver(pgpSchema),
        defaultValues: { display_name: '', public_key: '' },
    });

    const onSubmit = async (data: PgpForm) => {
        try {
            await enable.mutateAsync({ body: { display_name: data.display_name, public_key: data.public_key } });
            form.reset();
            setOpen(false);
            onRefetch();
        } catch {
            form.setError('public_key', { message: 'Invalid key or failed to add.' });
        }
    };

    const handleDisable = async () => {
        setDeleteError('');
        try {
            await disable.mutateAsync({});
            setConfirmDelete(false);
            setOpen(false);
            onRefetch();
        } catch {
            setDeleteError('Failed to remove.');
        }
    };

    const handleToggle = () => {
        if (isEnabled) {
            setOpen(v => !v);
            setConfirmDelete(false);
            setDeleteError('');
        } else {
            setOpen(v => !v);
        }
    };

    return (
        <FactorRow
            icon={<IconKey />}
            name="PGP Key"
            description="Authenticate by signing a server challenge with your PGP private key."
            tag={{ label: isEnabled ? pgp[0]?.display_name ?? 'Enabled' : 'Disabled', enabled: isEnabled }}
            onToggle={handleToggle}
            open={open}
        >
            {isEnabled ? (
                <ExpandForm open={open}>
                    <div className="ml-9 px-5 pb-4 space-y-2">
                        <div className="flex items-center gap-3 w-fit">
                            <div className="rounded-lg border border-border bg-muted/30 px-3 py-2">
                                <p className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground mb-1">Fingerprint</p>
                                <code className="font-mono text-[11px] text-foreground break-all">{pgp[0]?.fingerprint}</code>
                            </div>
                            {confirmDelete ? (
                                <div className="flex items-center gap-1.5 shrink-0">
                                    <button onClick={handleDisable} disabled={disable.isPending}
                                        className="text-xs text-destructive hover:text-destructive/80 font-medium transition-colors disabled:opacity-30">
                                        {disable.isPending ? 'Removing…' : 'Confirm'}
                                    </button>
                                    <button onClick={() => { setConfirmDelete(false); setDeleteError(''); }}
                                        className="text-xs text-muted-foreground hover:text-foreground transition-colors">
                                        Cancel
                                    </button>
                                </div>
                            ) : (
                                <button onClick={() => setConfirmDelete(true)}
                                    aria-label="Remove PGP key"
                                    className="shrink-0 text-muted-foreground hover:text-destructive transition-colors">
                                    <IconTrash size={13} />
                                </button>
                            )}
                        </div>
                        <ErrorMsg msg={deleteError} />
                    </div>
                </ExpandForm>
            ) : (
                <ExpandForm open={open}>
                    <Form {...form}>
                        <form onSubmit={form.handleSubmit(onSubmit)} className="ml-9 px-5 pb-4 space-y-3 max-w-sm">
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
                            <div className="flex gap-2">
                                <Button size="sm" type="submit" disabled={enable.isPending}>
                                    {enable.isPending ? 'Adding…' : 'Add key'}
                                </Button>
                                <Button size="sm" variant="ghost" type="button" onClick={() => { setOpen(false); form.reset(); }}>Cancel</Button>
                            </div>
                        </form>
                    </Form>
                </ExpandForm>
            )}
        </FactorRow>
    );
}
