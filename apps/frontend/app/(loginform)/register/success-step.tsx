'use client';

import { LoginIcon } from '@components/ui/login-icon';
import { IconArrowRight, IconCheck } from '@tabler/icons-react';
import { Button } from '@components/ui/button';

interface SuccessStepProps {
    onSignIn: () => void;
}

export function SuccessStep({ onSignIn }: SuccessStepProps) {
    return (
        <div className="flex flex-col items-center">
            <LoginIcon>
                <IconCheck />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Account created</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Your account has been created successfully.
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-4">
                <Button onClick={onSignIn}>
                    Sign In <IconArrowRight />
                </Button>
            </div>
        </div>
    );
}
