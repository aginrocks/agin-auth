'use client';

import { Form } from '@components/ui/form';
import { useForm } from 'react-hook-form';
import { LinkComponent } from '@components/ui/link';
import z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { $api } from '@lib/providers/api';
import { getApiErrorMessage } from '@lib/api-error';
import { useCallback, useState } from 'react';
import { AnimatePresence, motion } from 'motion/react';
import { useRouter } from 'next/navigation';
import { InfoStep } from './info-step';
import { CredentialsStep } from './credentials-step';
import { SuccessStep } from './success-step';

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

    const register = $api.useMutation('post', '/api/register');

    const handleNext = useCallback(async () => {
        const valid = await form.trigger(['first_name', 'last_name', 'display_name']);
        if (valid) setStep('credentials');
    }, []);

    const handleSubmit = form.handleSubmit(async (data) => {
        setError(null);
        try {
            const response = await register.mutateAsync({
                body: {
                    first_name: data.first_name,
                    last_name: data.last_name,
                    display_name: data.display_name,
                    preferred_username: data.preferred_username,
                    email: data.email,
                    password: data.password,
                },
            });

            if (!response?.success) {
                setError('Registration failed.');
                return;
            }

            setStep('success');
        } catch (error) {
            setError(getApiErrorMessage(error, 'Registration failed.'));
        }
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
                    {step === 'info' && <InfoStep onNext={handleNext} />}
                    {step === 'credentials' && (
                        <CredentialsStep
                            onBack={() => setStep('info')}
                            onSubmit={handleSubmit}
                            isPending={register.isPending}
                            error={error}
                        />
                    )}
                    {step === 'success' && <SuccessStep onSignIn={() => router.push('/login')} />}
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
