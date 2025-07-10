'use client';

import { Form } from '@components/ui/form';
import { useForm } from 'react-hook-form';
import { LinkComponent } from '@components/ui/link';
import z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { atom, useAtomValue } from 'jotai';
import { Welcome } from './welcome';
import { LoginOptions } from './login-options';
import { Password } from './password';

export const formSchema = z.object({
    username: z.string().min(1, 'Username is required'),
    password: z.string().optional(),
});

export type FormSchema = z.infer<typeof formSchema>;

export type LoginScreen =
    | 'welcome'
    | 'webauthn'
    | 'password'
    | 'totp'
    | 'gpg'
    | 'login-options'
    | 'recoverycode';

export const screenAtom = atom<LoginScreen>('welcome');

export default function Page() {
    const screen = useAtomValue(screenAtom);

    const form = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            username: '',
            password: '',
        },
    });

    return (
        <Form {...form}>
            {screen === 'welcome' && <Welcome />}
            {screen === 'login-options' && <LoginOptions />}
            {screen === 'password' && <Password />}
            <div className="text-muted-foreground text-xs absolute left-4 right-4 bottom-4 text-center">
                By signing in you accept our{' '}
                <LinkComponent>
                    <a>Privacy Policy</a>
                </LinkComponent>
            </div>
        </Form>
    );
}
