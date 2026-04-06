'use client';

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';
import { $api } from '@lib/providers/api';
import { getApiErrorMessage } from '@lib/api-error';
import { useQueryClient } from '@tanstack/react-query';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { IconUser, IconCheck, IconX } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { ErrorMsg, ExpandForm } from './helpers';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { Label } from '@components/ui/label';
import { useState } from 'react';

const profileSchema = z.object({
    display_name: z.string().min(1, 'Required').max(64),
    first_name: z.string().max(64).optional(),
    last_name: z.string().max(64).optional(),
});
type ProfileForm = z.infer<typeof profileSchema>;

interface Profile {
    preferred_username: string;
    display_name: string;
    first_name: string;
    last_name: string;
}

export function ProfileRow({ profile }: { profile: Profile }) {
    const queryClient = useQueryClient();
    const [open, setOpen] = useState(false);
    const [error, setError] = useState('');

    const form = useForm<ProfileForm>({
        resolver: zodResolver(profileSchema),
        values: {
            display_name: profile.display_name,
            first_name: profile.first_name,
            last_name: profile.last_name,
        },
    });

    const save = $api.useMutation('patch', '/api/settings/profile', {
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['get', '/api/settings/profile'] });
            setError('');
            setOpen(false);
        },
        onError: (error) => {
            setError(getApiErrorMessage(error, 'Failed to save profile changes.'));
        },
    });

    return (
        <FactorRow
            icon={<IconUser />}
            name={profile.display_name}
            description={[profile.first_name, profile.last_name].filter(Boolean).join(' ') || `@${profile.preferred_username}`}
            tag={{ label: `@${profile.preferred_username}`, enabled: true }}
            onToggle={() => {
                setOpen((v) => !v);
                form.reset();
                setError('');
            }}
            open={open}
            last
        >
            <ExpandForm open={open}>
                <Form {...form}>
                    <form
                        onSubmit={form.handleSubmit((data) => {
                            setError('');
                            save.mutate({ body: data });
                        })}
                        className="ml-9 px-5 pb-4 space-y-3 max-w-sm"
                    >
                        <div className="grid grid-cols-2 gap-3">
                            <FormField control={form.control} name="first_name" render={({ field }) => (
                                <FormItem className="space-y-1">
                                    <Label className="text-xs">First name</Label>
                                    <FormControl>
                                        <Input {...field} value={field.value ?? ''} placeholder="First name" className="h-8 text-sm" />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )} />
                            <FormField control={form.control} name="last_name" render={({ field }) => (
                                <FormItem className="space-y-1">
                                    <Label className="text-xs">Last name</Label>
                                    <FormControl>
                                        <Input {...field} value={field.value ?? ''} placeholder="Last name" className="h-8 text-sm" />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )} />
                        </div>
                        <FormField control={form.control} name="display_name" render={({ field }) => (
                            <FormItem className="space-y-1">
                                <Label className="text-xs">Display name</Label>
                                <FormControl>
                                    <Input {...field} placeholder="Display name" className="h-8 text-sm" />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )} />
                        <ErrorMsg msg={error} />
                        <div className="flex gap-2">
                            <Button size="sm" type="submit" disabled={save.isPending}>
                                <IconCheck size={14} /> {save.isPending ? 'Saving…' : 'Save'}
                            </Button>
                            <Button
                                variant="ghost"
                                size="sm"
                                type="button"
                                onClick={() => {
                                    setOpen(false);
                                    form.reset();
                                    setError('');
                                }}
                            >
                                <IconX size={14} /> Cancel
                            </Button>
                        </div>
                    </form>
                </Form>
            </ExpandForm>
        </FactorRow>
    );
}
