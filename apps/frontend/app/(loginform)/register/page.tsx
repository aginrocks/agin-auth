'use client';

import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@components/ui/form';
import { useForm } from 'react-hook-form';
import { LinkComponent } from '@components/ui/link';
import Link from 'next/link';
import z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { LoginIcon } from '@components/ui/login-icon';
import { IconArrowRight, IconCheck, IconUserPlus } from '@tabler/icons-react';
import { Input } from '@components/ui/input';
import { Button } from '@components/ui/button';
import { $api } from '@lib/providers/api';
import { useState } from 'react';
import { AnimatePresence, motion } from 'motion/react';
import { Alert, AlertDescription, AlertTitle } from '@components/ui/alert';
import { useRouter } from 'next/navigation';

const registerSchema = z
    .object({
        first_name: z.string().min(1, 'First name is required').max(32),
        last_name: z.string().min(1, 'Last name is required').max(32),
        display_name: z.string().min(1, 'Display name is required').max(32),
        preferred_username: z
            .string()
            .min(1, 'Username is required')
            .max(32)
            .regex(/^[a-zA-Z0-9_.-]+$/, 'Only letters, numbers, dots, hyphens and underscores'),
        email: z.string().email('Invalid email address').max(128),
        password: z.string().min(8, 'Password must be at least 8 characters'),
        confirm_password: z.string(),
    })
    .refine((data) => data.password === data.confirm_password, {
        message: 'Passwords do not match',
        path: ['confirm_password'],
    });

type RegisterSchema = z.infer<typeof registerSchema>;

type Step = 'info' | 'credentials' | 'success';

export default function Page() {
    const [step, setStep] = useState<Step>('info');
    const [error, setError] = useState<string | null>(null);
    const router = useRouter();

    const form = useForm<RegisterSchema>({
        resolver: zodResolver(registerSchema),
        defaultValues: {
            first_name: '',
            last_name: '',
            display_name: '',
            preferred_username: '',
            email: '',
            password: '',
            confirm_password: '',
        },
    });

    const register = $api.useMutation('post', '/api/register', {
        onSuccess: () => {
            setStep('success');
            setError(null);
        },
        onError: (e) => {
            setError(e?.error || 'Registration failed.');
        },
    });

    const handleNext = async () => {
        const valid = await form.trigger(['first_name', 'last_name', 'display_name']);
        if (valid) setStep('credentials');
    };

    const handleSubmit = form.handleSubmit((data) => {
        setError(null);
        register.mutate({
            body: {
                first_name: data.first_name,
                last_name: data.last_name,
                display_name: data.display_name,
                preferred_username: data.preferred_username,
                email: data.email,
                password: data.password,
            },
        });
    });

    return (
        <Form {...form}>
            <AnimatePresence mode="wait" initial={false}>
                <motion.div
                    key={step}
                    transition={{ duration: 0.2 }}
                    initial={{ opacity: 0, x: 8 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, x: -8 }}
                >
                    {step === 'info' && (
                        <div className="flex flex-col items-center">
                            <LoginIcon>
                                <IconUserPlus />
                            </LoginIcon>
                            <div className="mt-4 flex flex-col gap-1">
                                <h1 className="font-semibold text-xl text-center">
                                    Create your account
                                </h1>
                                <p className="text-sm text-center text-muted-foreground">
                                    Tell us a bit about yourself
                                </p>
                            </div>
                            <div className="w-sm mt-6 flex flex-col gap-4">
                                <div className="flex gap-3">
                                    <FormField
                                        control={form.control}
                                        name="first_name"
                                        render={({ field }) => (
                                            <FormItem className="flex-1">
                                                <FormLabel>First Name</FormLabel>
                                                <FormControl>
                                                    <Input
                                                        placeholder="John"
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
                                        name="last_name"
                                        render={({ field }) => (
                                            <FormItem className="flex-1">
                                                <FormLabel>Last Name</FormLabel>
                                                <FormControl>
                                                    <Input placeholder="Doe" {...field} />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                </div>
                                <FormField
                                    control={form.control}
                                    name="display_name"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Display Name</FormLabel>
                                            <FormControl>
                                                <Input placeholder="John Doe" {...field} />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <Button type="button" onClick={handleNext}>
                                    Next <IconArrowRight />
                                </Button>
                                <div className="text-muted-foreground text-center text-sm">
                                    Already have an account?{' '}
                                    <LinkComponent>
                                        <Link href="/login">Sign In</Link>
                                    </LinkComponent>
                                </div>
                            </div>
                        </div>
                    )}

                    {step === 'credentials' && (
                        <form
                            className="flex flex-col items-center"
                            onSubmit={handleSubmit}
                        >
                            <LoginIcon>
                                <IconUserPlus />
                            </LoginIcon>
                            <div className="mt-4 flex flex-col gap-1">
                                <h1 className="font-semibold text-xl text-center">
                                    Set up your credentials
                                </h1>
                                <p className="text-sm text-center text-muted-foreground">
                                    Choose a username and password
                                </p>
                            </div>
                            <div className="w-sm mt-6 flex flex-col gap-4">
                                {error && (
                                    <Alert variant="destructive">
                                        <AlertTitle>Registration Failed</AlertTitle>
                                        <AlertDescription>{error}</AlertDescription>
                                    </Alert>
                                )}
                                <FormField
                                    control={form.control}
                                    name="preferred_username"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Username</FormLabel>
                                            <FormControl>
                                                <Input
                                                    placeholder="johndoe"
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
                                    name="email"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Email</FormLabel>
                                            <FormControl>
                                                <Input
                                                    placeholder="john@example.com"
                                                    type="email"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <FormField
                                    control={form.control}
                                    name="password"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Password</FormLabel>
                                            <FormControl>
                                                <Input
                                                    placeholder="Enter your password"
                                                    type="password"
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
                                            <FormLabel>Confirm Password</FormLabel>
                                            <FormControl>
                                                <Input
                                                    placeholder="Confirm your password"
                                                    type="password"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <div className="flex gap-3">
                                    <Button
                                        type="button"
                                        variant="outline"
                                        className="flex-1"
                                        onClick={() => setStep('info')}
                                    >
                                        Back
                                    </Button>
                                    <Button
                                        type="submit"
                                        className="flex-1"
                                        disabled={register.isPending}
                                    >
                                        Create Account <IconArrowRight />
                                    </Button>
                                </div>
                                <div className="text-muted-foreground text-center text-sm">
                                    Already have an account?{' '}
                                    <LinkComponent>
                                        <Link href="/login">Sign In</Link>
                                    </LinkComponent>
                                </div>
                            </div>
                        </form>
                    )}

                    {step === 'success' && (
                        <div className="flex flex-col items-center">
                            <LoginIcon>
                                <IconCheck />
                            </LoginIcon>
                            <div className="mt-4 flex flex-col gap-1">
                                <h1 className="font-semibold text-xl text-center">
                                    Account created
                                </h1>
                                <p className="text-sm text-center text-muted-foreground">
                                    Your account has been created successfully.
                                </p>
                            </div>
                            <div className="w-sm mt-6 flex flex-col gap-4">
                                <Button onClick={() => router.push('/login')}>
                                    Sign In <IconArrowRight />
                                </Button>
                            </div>
                        </div>
                    )}
                </motion.div>
            </AnimatePresence>
            <div className="text-muted-foreground text-xs absolute left-4 right-4 bottom-4 text-center">
                By signing up you accept our{' '}
                <LinkComponent>
                    <a>Privacy Policy</a>
                </LinkComponent>
            </div>
        </Form>
    );
}
