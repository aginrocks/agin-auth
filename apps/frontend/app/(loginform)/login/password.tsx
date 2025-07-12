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
import { twofactorOptionsAtom } from './two-factor-options';

export function Password() {
    const setScreen = useSetAtom(screenAtom);
    const setOptions = useSetAtom(twofactorOptionsAtom);

    const form = useFormContext<FormSchema>();

    const username = form.watch('username');

    const passwordLogin = $api.useMutation('post', '/api/login/password', {
        onSuccess: ({ two_factor_required, second_factors, recent_factor }) => {
            if (!two_factor_required || !second_factors) return;

            setOptions(second_factors);

            if (second_factors.length === 1) return setScreen(second_factors[0]);
            if (recent_factor) return setScreen(recent_factor);
            setScreen('two-factor-options');
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
                        <div onClick={() => setScreen('login-options')}>More Options</div>
                    </LinkComponent>
                </div>
            </div>
        </form>
    );
}
