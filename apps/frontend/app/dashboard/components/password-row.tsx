'use client';

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';
import { $api } from '@lib/providers/api';
import { getApiErrorMessage } from '@lib/api-error';
import { Button } from '@components/ui/button';
import { Label } from '@components/ui/label';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@components/ui/dialog';
import { IconLock } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { ErrorMsg, PasswordInput } from './helpers';

const passwordSchema = z.object({
    current_password: z.string().optional(),
    new_password: z.string().min(8, 'Min. 8 characters'),
    confirm_password: z.string(),
}).refine(data => data.new_password === data.confirm_password, {
    message: 'Passwords do not match',
    path: ['confirm_password'],
});
type PasswordForm = z.infer<typeof passwordSchema>;

export function PasswordRow({ isSet, onRefetch }: { isSet: boolean; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [error, setError] = useState('');

    const form = useForm<PasswordForm>({
        resolver: zodResolver(passwordSchema),
        defaultValues: { current_password: '', new_password: '', confirm_password: '' },
    });

    const change = $api.useMutation('post', '/api/settings/password/change', {
        onSuccess: () => {
            form.reset();
            setError('');
            setOpen(false);
            onRefetch();
        },
        onError: (error) => {
            setError(getApiErrorMessage(error, 'Could not update the password.'));
        },
    });

    const onSubmit = (data: PasswordForm) => {
        setError('');
        change.mutate({ body: { current_password: data.current_password ?? '', new_password: data.new_password } });
    };

    const handleClose = (v: boolean) => {
        if (!v) { form.reset(); setError(''); }
        setOpen(v);
    };

    return (
        <>
            <FactorRow icon={<IconLock />} name="Password" description="Authenticate using a password." tag={{ label: isSet ? 'Enabled' : 'Disabled', enabled: isSet }} onToggle={() => setOpen(true)} open={false}>
                <></>
            </FactorRow>

            <Dialog open={open} onOpenChange={handleClose}>
                <DialogContent className="sm:max-w-md">
                    <DialogHeader>
                        <DialogTitle>{isSet ? 'Change password' : 'Set password'}</DialogTitle>
                        <DialogDescription>
                            {isSet ? 'Enter your current password and choose a new one.' : 'Set a password for your account.'}
                        </DialogDescription>
                    </DialogHeader>
                    <Form {...form}>
                        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-3">
                            {isSet && (
                                <FormField
                                    control={form.control}
                                    name="current_password"
                                    render={({ field }) => (
                                        <FormItem className="space-y-1.5">
                                            <Label htmlFor="current-password" className="text-xs">Current password</Label>
                                            <FormControl>
                                                <PasswordInput id="current-password" value={field.value ?? ''} onChange={field.onChange} required />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                            )}
                            <FormField
                                control={form.control}
                                name="new_password"
                                render={({ field }) => (
                                    <FormItem className="space-y-1.5">
                                        <Label htmlFor="new-password" className="text-xs">New password</Label>
                                        <FormControl>
                                            <PasswordInput id="new-password" value={field.value} onChange={field.onChange} minLength={8} required placeholder="Min. 8 characters" />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="confirm_password"
                                render={({ field }) => (
                                    <FormItem className="space-y-1.5">
                                        <Label htmlFor="confirm-password" className="text-xs">Confirm new password</Label>
                                        <FormControl>
                                            <PasswordInput id="confirm-password" value={field.value} onChange={field.onChange} minLength={8} required placeholder="Repeat new password" />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <ErrorMsg msg={error} />
                            <div className="flex gap-2 pt-1 justify-end">
                                <Button variant="outline" type="button" onClick={() => handleClose(false)}>
                                    Cancel
                                </Button>
                                <Button type="submit" disabled={change.isPending}>
                                    {change.isPending ? 'Saving…' : isSet ? 'Update password' : 'Set password'}
                                </Button>
                            </div>
                        </form>
                    </Form>
                </DialogContent>
            </Dialog>
        </>
    );
}
