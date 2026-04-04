'use client';

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { Label } from '@components/ui/label';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { IconLock } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { ErrorMsg, ExpandForm, PasswordInput } from './helpers';

const passwordSchema = z.object({
    current_password: z.string().optional(),
    new_password: z.string().min(8, 'Min. 8 characters'),
});
type PasswordForm = z.infer<typeof passwordSchema>;

export function PasswordRow({ isSet, onRefetch }: { isSet: boolean; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [error, setError] = useState('');
    const change = $api.useMutation('post', '/api/settings/password/change');

    const form = useForm<PasswordForm>({
        resolver: zodResolver(passwordSchema),
        defaultValues: { current_password: '', new_password: '' },
    });

    const onSubmit = async (data: PasswordForm) => {
        setError('');
        try {
            await change.mutateAsync({ body: { current_password: data.current_password ?? '', new_password: data.new_password } });
            form.reset();
            setOpen(false);
            onRefetch();
        } catch {
            setError('Incorrect current password or invalid new password.');
        }
    };

    return (
        <FactorRow icon={<IconLock />} name="Password" description="Authenticate using a password." tag={{ label: isSet ? 'Enabled' : 'Disabled', enabled: isSet }} onToggle={() => setOpen(v => !v)} open={open}>
            <ExpandForm open={open}>
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-3 max-w-sm ml-9 px-5 pb-4">
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
                        <ErrorMsg msg={error} />
                        <div className="flex gap-2 pt-1">
                            <Button size="sm" type="submit" disabled={change.isPending}>
                                {change.isPending ? 'Saving…' : isSet ? 'Update' : 'Set password'}
                            </Button>
                            <Button size="sm" variant="ghost" type="button" onClick={() => { setOpen(false); setError(''); }}>
                                Cancel
                            </Button>
                        </div>
                    </form>
                </Form>
            </ExpandForm>
        </FactorRow>
    );
}
