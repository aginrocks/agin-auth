import { FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { LoginIcon } from '@components/ui/login-icon';
import { IconArrowRight, IconLifebuoy } from '@tabler/icons-react';
import { useFormContext } from 'react-hook-form';
import { FormSchema, screenAtom } from './page';
import { Input } from '@components/ui/input';
import { Button } from '@components/ui/button';
import { LinkComponent } from '@components/ui/link';
import { $api } from '@lib/providers/api';
import { useSetAtom } from 'jotai';
import { useRouter } from 'next/navigation';
import { useLoginSuccess } from '@lib/hooks';

export function RecoveryCode() {
    const setScreen = useSetAtom(screenAtom);
    const router = useRouter();

    const form = useFormContext<FormSchema>();

    const { onSuccess } = useLoginSuccess();

    const recoveryCodeLogin = $api.useMutation('post', '/api/login/recovery-codes', {
        onSuccess,
        onError: (e) => {
            form.setError('recovery_code', {
                message: e?.error || 'Login failed.',
            });
        },
    });

    return (
        <form
            className="flex flex-col items-center"
            onSubmit={form.handleSubmit((data) =>
                recoveryCodeLogin.mutate({
                    body: {
                        code: data.recovery_code ?? '',
                    },
                })
            )}
        >
            <LoginIcon>
                <IconLifebuoy />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Two-factor recovery</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Enter one of your recovery codes.
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-4">
                <FormField
                    control={form.control}
                    name="recovery_code"
                    render={({ field }) => (
                        <FormItem>
                            <FormControl>
                                <Input
                                    placeholder="Enter your recovery code"
                                    type="password"
                                    autoFocus
                                    {...field}
                                />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button type="submit" disabled={recoveryCodeLogin.isPending}>
                    Next <IconArrowRight />
                </Button>
                <div className="text-muted-foreground text-center text-sm">
                    <LinkComponent>
                        <div onClick={() => setScreen('two-factor-options')}>More Options</div>
                    </LinkComponent>
                </div>
            </div>
        </form>
    );
}
