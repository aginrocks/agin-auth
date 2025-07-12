import { FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { LoginIcon } from '@components/ui/login-icon';
import { IconClock } from '@tabler/icons-react';
import { useFormContext } from 'react-hook-form';
import { FormSchema, screenAtom } from './page';
import { LinkComponent } from '@components/ui/link';
import { $api } from '@lib/providers/api';
import { useSetAtom } from 'jotai';
import { InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot } from '@components/ui/input-otp';
import { REGEXP_ONLY_DIGITS } from 'input-otp';
import { useCallback, useEffect } from 'react';

export function Totp() {
    const setScreen = useSetAtom(screenAtom);

    const form = useFormContext<FormSchema>();

    const code = form.watch('totp');

    const totpLogin = $api.useMutation('post', '/api/login/totp', {
        onSuccess: ({ two_factor_required, second_factors }) => {
            alert('Success!');
        },
        onError: (e) => {
            form.setError('totp', {
                message: e?.error || 'Login failed.',
            });
        },
    });

    useEffect(() => {
        trySubmitCode(code);
    }, [code]);

    const trySubmitCode = useCallback((code: string | undefined) => {
        if (code?.length !== 6) return;

        totpLogin.mutate({
            body: {
                code: code,
            },
        });
    }, []);

    return (
        <form
            className="flex flex-col items-center"
            onSubmit={form.handleSubmit((values) => trySubmitCode(values.totp))}
        >
            <LoginIcon>
                <IconClock />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">One-time Code</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Enter the code from your two-factor authentication app
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-4">
                <div className="flex justify-center mb-1">
                    <FormField
                        control={form.control}
                        name="totp"
                        render={({ field }) => (
                            <FormItem>
                                <FormControl>
                                    <InputOTP maxLength={6} pattern={REGEXP_ONLY_DIGITS} {...field}>
                                        <InputOTPGroup>
                                            <InputOTPSlot index={0} />
                                            <InputOTPSlot index={1} />
                                            <InputOTPSlot index={2} />
                                        </InputOTPGroup>
                                        <InputOTPSeparator />
                                        <InputOTPGroup>
                                            <InputOTPSlot index={3} />
                                            <InputOTPSlot index={4} />
                                            <InputOTPSlot index={5} />
                                        </InputOTPGroup>
                                    </InputOTP>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                </div>
                <div className="text-muted-foreground text-center text-sm">
                    <LinkComponent>
                        <div onClick={() => setScreen('two-factor-options')}>More Options</div>
                    </LinkComponent>
                </div>
            </div>
        </form>
    );
}
