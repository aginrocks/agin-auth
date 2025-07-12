import { FormControl, FormField, FormItem, FormLabel, FormMessage } from '@components/ui/form';
import { LoginIcon } from '@components/ui/login-icon';
import {
    IconArrowRight,
    IconFingerprint,
    IconKey,
    IconPassword,
    IconShieldLock,
} from '@tabler/icons-react';
import { useFormContext } from 'react-hook-form';
import { FormSchema, screenAtom } from './page';
import { Input } from '@components/ui/input';
import { Button } from '@components/ui/button';
import { LinkComponent } from '@components/ui/link';
import Link from 'next/link';
import { $api } from '@lib/providers/api';
import { Separator } from '@components/ui/separator';
import { atom, useAtomValue, useSetAtom } from 'jotai';
import { paths } from 'api-schema';
import { LoginOption, LoginOptionProps } from '@components/ui/login-option';

export type TLoginOption =
    paths['/api/login/options']['get']['responses']['200']['content']['application/json']['options'][number];

export const optionsAtom = atom<TLoginOption[]>();

export const OPTIONS_MAP: Record<TLoginOption, LoginOptionProps> = {
    password: {
        title: 'Password',
        icon: IconPassword,
        rightSection: <IconArrowRight className="size-4 text-muted-foreground" />,
    },
    webauthn: {
        title: 'Security key / Passkey',
        icon: IconFingerprint,
        rightSection: <IconArrowRight className="size-4 text-muted-foreground" />,
    },
    pgp: {
        title: 'PGP Key',
        icon: IconKey,
        rightSection: <IconArrowRight className="size-4 text-muted-foreground" />,
    },
};

export function LoginOptions() {
    const setScreen = useSetAtom(screenAtom);
    const options = useAtomValue(optionsAtom);

    return (
        <div className="flex flex-col items-center">
            <LoginIcon>
                <IconShieldLock />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Choose Authentication Method</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Select how you'd like to verify your identity to continue
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-3">
                {options?.map((o) => (
                    <LoginOption
                        {...OPTIONS_MAP[o]}
                        clickable
                        key={o}
                        className="m-0"
                        onClick={() => setScreen(o)}
                    />
                ))}
            </div>
        </div>
    );
}
