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
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';

const schema = z.object({
    email: z.string().email('Invalid email address'),
});
type Schema = z.infer<typeof schema>;

type Step = 'form' | 'sent';

export default function Page() {
    const [step, setStep] = useState<Step>('form');

    const form = useForm<Schema>({
        resolver: zodResolver(schema),
        defaultValues: { email: '' },
    });

    const requestReset = $api.useMutation('post', '/api/password-reset', {
        onSuccess: () => setStep('sent'),
        onError: (e) => {
            form.setError('email', { message: (e as any)?.error || 'Something went wrong.' });
        },
    });

    return (
        <div className="flex flex-col items-center">
            <AnimatePresence mode="wait" initial={false}>
                {step === 'form' ? (
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
                                    requestReset.mutate({ body: { email: data.email } })
                                )}
                            >
                                <LoginIcon>
                                    <IconLock />
                                </LoginIcon>
                                <div className="mt-4 flex flex-col gap-1">
                                    <h1 className="font-semibold text-xl text-center">Reset your password</h1>
                                    <p className="text-sm text-center text-muted-foreground">
                                        Enter your email and we&apos;ll send you a reset link.
                                    </p>
                                </div>
                                <div className="w-sm mt-6 flex flex-col gap-4">
                                    <FormField
                                        control={form.control}
                                        name="email"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormControl>
                                                    <Input
                                                        placeholder="you@example.com"
                                                        type="email"
                                                        autoFocus
                                                        {...field}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                    <Button type="submit" disabled={requestReset.isPending}>
                                        Send reset link <IconArrowRight />
                                    </Button>
                                    <div className="text-muted-foreground text-center text-sm">
                                        <LinkComponent>
                                            <Link href="/login">Back to sign in</Link>
                                        </LinkComponent>
                                    </div>
                                </div>
                            </form>
                        </Form>
                    </motion.div>
                ) : (
                    <motion.div
                        key="sent"
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
                            <h1 className="font-semibold text-xl text-center">Check your email</h1>
                            <p className="text-sm text-center text-muted-foreground max-w-xs">
                                If an account with that address exists, we&apos;ve sent a reset link. It expires in 1 hour.
                            </p>
                        </div>
                        <div className="w-sm mt-6 flex flex-col items-center">
                            <LinkComponent>
                                <Link href="/login">Back to sign in</Link>
                            </LinkComponent>
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>
        </div>
    );
}
