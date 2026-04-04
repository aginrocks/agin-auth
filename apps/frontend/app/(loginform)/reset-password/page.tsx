'use client';

import { Button } from '@components/ui/button';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { Input } from '@components/ui/input';
import { LoginIcon } from '@components/ui/login-icon';
import { LinkComponent } from '@components/ui/link';
import { $api } from '@lib/providers/api';
import { IconArrowRight, IconLock } from '@tabler/icons-react';
import { AnimatePresence, motion } from 'motion/react';
import Link from 'next/link';
import { useRouter, useSearchParams } from 'next/navigation';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';

const schema = z
    .object({
        new_password: z.string().min(8, 'Password must be at least 8 characters'),
        confirm_password: z.string(),
    })
    .refine((d) => d.new_password === d.confirm_password, {
        message: 'Passwords do not match',
        path: ['confirm_password'],
    });

type Schema = z.infer<typeof schema>;

export default function Page() {
    const searchParams = useSearchParams();
    const token = searchParams.get('token') ?? '';
    const router = useRouter();
    const [done, setDone] = useState(false);

    const form = useForm<Schema>({
        resolver: zodResolver(schema),
        defaultValues: { new_password: '', confirm_password: '' },
    });

    const confirmReset = $api.useMutation('post', '/api/password-reset/confirm', {
        onSuccess: () => setDone(true),
        onError: (e) => {
            form.setError('new_password', {
                message: (e as any)?.error || 'Invalid or expired link.',
            });
        },
    });

    if (!token) {
        return (
            <div className="flex flex-col items-center gap-4">
                <LoginIcon>
                    <IconLock />
                </LoginIcon>
                <p className="text-sm text-muted-foreground text-center">
                    Invalid reset link.{' '}
                    <LinkComponent>
                        <Link href="/forgot-password">Request a new one.</Link>
                    </LinkComponent>
                </p>
            </div>
        );
    }

    return (
        <div className="flex flex-col items-center">
            <AnimatePresence mode="wait" initial={false}>
                {!done ? (
                    <motion.div
                        key="form"
                        className="flex flex-col items-center w-full"
                        transition={{ duration: 0.2 }}
                        initial={{ opacity: 0, x: 8 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: -8 }}
                    >
                        <Form {...form}>
                            <form
                                className="flex flex-col items-center w-full"
                                onSubmit={form.handleSubmit((data) =>
                                    confirmReset.mutate({
                                        body: { token, new_password: data.new_password },
                                    })
                                )}
                            >
                                <LoginIcon>
                                    <IconLock />
                                </LoginIcon>
                                <div className="mt-4 flex flex-col gap-1">
                                    <h1 className="font-semibold text-xl text-center">Set new password</h1>
                                    <p className="text-sm text-center text-muted-foreground">
                                        Choose a strong password for your account.
                                    </p>
                                </div>
                                <div className="w-sm mt-6 flex flex-col gap-4">
                                    <FormField
                                        control={form.control}
                                        name="new_password"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormControl>
                                                    <Input
                                                        placeholder="New password"
                                                        type="password"
                                                        autoFocus
                                                        {...field}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                    <FormField
                                        control={form.control}
                                        name="confirm_password"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormControl>
                                                    <Input
                                                        placeholder="Confirm new password"
                                                        type="password"
                                                        {...field}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                    <Button type="submit" disabled={confirmReset.isPending}>
                                        Set password <IconArrowRight />
                                    </Button>
                                </div>
                            </form>
                        </Form>
                    </motion.div>
                ) : (
                    <motion.div
                        key="done"
                        className="flex flex-col items-center w-full"
                        transition={{ duration: 0.2 }}
                        initial={{ opacity: 0, x: 8 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: -8 }}
                    >
                        <LoginIcon>
                            <IconLock />
                        </LoginIcon>
                        <div className="mt-4 flex flex-col gap-1">
                            <h1 className="font-semibold text-xl text-center">Password changed!</h1>
                            <p className="text-sm text-center text-muted-foreground">
                                You can now sign in with your new password.
                            </p>
                        </div>
                        <div className="w-sm mt-6 flex flex-col items-center">
                            <Button onClick={() => router.push('/login')}>
                                Sign in <IconArrowRight />
                            </Button>
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>
        </div>
    );
}
