'use client';
import { Button } from '@components/ui/button';
import { useWebAuthnRegistration } from '@lib/hooks';

export default function Page() {
    const webAuthn = useWebAuthnRegistration();

    return (
        <div>
            Welcome!
            <Button onClick={() => webAuthn.registerAsync('test')}>Register</Button>
        </div>
    );
}
