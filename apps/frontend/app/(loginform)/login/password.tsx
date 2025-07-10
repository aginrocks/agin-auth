import { FormControl, FormField, FormItem, FormLabel, FormMessage } from '@components/ui/form';
import { LoginIcon } from '@components/ui/login-icon';
import { IconArrowLeft, IconArrowRight, IconKey, IconPassword } from '@tabler/icons-react';
import { useFormContext } from 'react-hook-form';
import { FormSchema, screenAtom } from './page';
import { Input } from '@components/ui/input';
import { Button } from '@components/ui/button';
import { LinkComponent } from '@components/ui/link';
import Link from 'next/link';
import { $api } from '@lib/providers/api';
import { Separator } from '@components/ui/separator';
import { useSetAtom } from 'jotai';
import { optionsAtom } from './login-options';

export function Password() {
    const setScreen = useSetAtom(screenAtom);

    const form = useFormContext<FormSchema>();

    const username = form.watch('username');

    const passwordLogin = $api.useMutation('post', '/api/login/password', {
        onSuccess: ({ two_factor_required, second_factors }) => {
            // if (options.length === 1) return setScreen(options[0]);
            // setScreen('login-options');
        },
        onError: (e) => {
            form.setError('password', {
                message: e?.error || 'Login failed.',
            });
        },
    });

    return (
        <form
            className="flex flex-col items-center"
            onSubmit={form.handleSubmit((data) =>
                passwordLogin.mutate({
                    body: {
                        username: data.username,
                        password: data.password ?? '',
                    },
                })
            )}
        >
            <LoginIcon>
                <IconPassword />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Enter Your Password</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Logging in as {username}{' '}
                    <LinkComponent onClick={() => setScreen('welcome')}>Not you?</LinkComponent>
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-4">
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
                                    autoFocus
                                    {...field}
                                />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button type="submit" disabled={passwordLogin.isPending}>
                    Next <IconArrowRight />
                </Button>
                <div className="text-muted-foreground text-center text-sm">
                    <LinkComponent>
                        <Link href="/register">Forgot Password?</Link>
                    </LinkComponent>
                </div>
            </div>
        </form>
    );
}
