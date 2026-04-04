'use client';

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';
import { $api } from '@lib/providers/api';
import { useRouter } from 'next/navigation';
import { Button } from '@components/ui/button';
import { IconTrash } from '@tabler/icons-react';
import { ExpandForm, PasswordInput } from './helpers';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { useState } from 'react';

const deleteSchema = z.object({
    password: z.string().min(1, 'Required'),
});
type DeleteForm = z.infer<typeof deleteSchema>;

export function DeleteAccountSection() {
    const router = useRouter();
    const [open, setOpen] = useState(false);

    const form = useForm<DeleteForm>({
        resolver: zodResolver(deleteSchema),
        defaultValues: { password: '' },
    });

    const deleteAccount = $api.useMutation('delete', '/api/settings/account', {
        onSuccess: () => router.push('/login'),
        onError: () => {
            form.setError('password', { message: 'Incorrect password or failed to delete.' });
        },
    });

    return (
        <div className="px-5 py-4">
            <div className="flex items-center justify-between">
                <div>
                    <h3 className="text-sm font-medium">Delete account</h3>
                    <p className="text-xs text-muted-foreground">Permanently remove your account and all associated data.</p>
                </div>
                {!open && (
                    <Button variant="destructive" size="sm" onClick={() => setOpen(true)}>
                        <IconTrash size={14} /> Delete
                    </Button>
                )}
            </div>
            <ExpandForm open={open}>
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(data => deleteAccount.mutate({ body: data }))} className="mt-1 space-y-3 max-w-sm pb-1">
                        <p className="text-xs text-destructive">This action is irreversible. Enter your password to confirm.</p>
                        <FormField control={form.control} name="password" render={({ field }) => (
                            <FormItem>
                                <FormControl>
                                    <PasswordInput value={field.value} onChange={field.onChange} placeholder="Your password" required />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )} />
                        <div className="flex gap-2">
                            <Button variant="destructive" size="sm" type="submit" disabled={deleteAccount.isPending}>
                                {deleteAccount.isPending ? 'Deleting…' : 'Delete my account'}
                            </Button>
                            <Button variant="ghost" size="sm" type="button" onClick={() => { setOpen(false); form.reset(); }}>
                                Cancel
                            </Button>
                        </div>
                    </form>
                </Form>
            </ExpandForm>
        </div>
    );
}
