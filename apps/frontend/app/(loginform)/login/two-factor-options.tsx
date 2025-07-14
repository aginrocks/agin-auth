import { LoginIcon } from '@components/ui/login-icon';
import {
    IconArrowRight,
    IconClock,
    IconFingerprint,
    IconKey,
    IconLifebuoy,
    IconShieldLock,
} from '@tabler/icons-react';
import { screenAtom } from './page';
import { atom, useAtomValue, useSetAtom } from 'jotai';
import { paths } from 'api-schema';
import { LoginOption, LoginOptionProps } from '@components/ui/login-option';

export type T2FAOption = Exclude<
    paths['/api/login/password']['post']['responses']['200']['content']['application/json']['second_factors'],
    undefined | null
>[number];

export const twofactorOptionsAtom = atom<T2FAOption[]>();

export const OPTIONS_MAP: Record<T2FAOption, LoginOptionProps> = {
    webauthn: {
        title: 'Security key / Passkey',
        icon: IconFingerprint,
        rightSection: <IconArrowRight className="size-4 text-muted-foreground" />,
    },
    totp: {
        title: 'One-time Code',
        icon: IconClock,
        rightSection: <IconArrowRight className="size-4 text-muted-foreground" />,
    },
    recoverycode: {
        title: 'Recovery Code',
        icon: IconLifebuoy,
        rightSection: <IconArrowRight className="size-4 text-muted-foreground" />,
    },
};

export function TwoFactorOptions() {
    const setScreen = useSetAtom(screenAtom);
    const options = useAtomValue(twofactorOptionsAtom);

    return (
        <div className="flex flex-col items-center">
            <LoginIcon>
                <IconShieldLock />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">
                    Continue with Two-Factor Authentication
                </h1>
                <p className="text-sm text-center text-muted-foreground">
                    Select a method to verify your identity
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
