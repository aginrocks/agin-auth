'use client';

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { Label } from '@components/ui/label';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { IconDeviceMobile, IconCheck } from '@tabler/icons-react';
import QRCode from 'react-qr-code';
import { FactorRow } from './factor-row';
import { CopyButton, ErrorMsg, ExpandForm } from './helpers';

const nameSchema = z.object({
    display_name: z.string().min(1, 'Required').max(32),
});
const codeSchema = z.object({
    code: z.string().length(6, 'Must be 6 digits'),
});

type NameForm = z.infer<typeof nameSchema>;
type CodeForm = z.infer<typeof codeSchema>;

export function TotpRow({ totp, onRefetch }: { totp: { display_name: string; fully_enabled: boolean } | null | undefined; onRefetch: () => void }) {
    const [step, setStep] = useState<'idle' | 'name' | 'confirm'>('idle');
    const [setupData, setSetupData] = useState<{ secret: string; qr: string } | null>(null);
    const [disableError, setDisableError] = useState('');

    const enable = $api.useMutation('post', '/api/settings/factors/totp/enable');
    const confirm = $api.useMutation('post', '/api/settings/factors/totp/enable/confirm');
    const disable = $api.useMutation('delete', '/api/settings/factors/totp/disable');

    const isEnabled = totp?.fully_enabled ?? false;

    const nameForm = useForm<NameForm>({
        resolver: zodResolver(nameSchema),
        defaultValues: { display_name: '' },
    });

    const codeForm = useForm<CodeForm>({
        resolver: zodResolver(codeSchema),
        defaultValues: { code: '' },
    });

    const onNameSubmit = async (data: NameForm) => {
        try {
            const d = await enable.mutateAsync({ body: { display_name: data.display_name } });
            setSetupData(d);
            setStep('confirm');
        } catch {
            nameForm.setError('display_name', { message: 'Failed to start setup.' });
        }
    };

    const onCodeSubmit = async (data: CodeForm) => {
        try {
            await confirm.mutateAsync({ body: { code: data.code } });
            setStep('idle');
            setSetupData(null);
            nameForm.reset();
            codeForm.reset();
            onRefetch();
        } catch {
            codeForm.setError('code', { message: 'Invalid code, try again.' });
        }
    };

    const handleDisable = async () => {
        setDisableError('');
        try {
            await disable.mutateAsync({});
            onRefetch();
        } catch {
            setDisableError('Failed to disable.');
        }
    };

    const handleToggle = () => {
        if (step !== 'idle') {
            setStep('idle');
            setSetupData(null);
            nameForm.reset();
            codeForm.reset();
        } else {
            setStep(isEnabled ? 'idle' : 'name');
        }
    };

    return (
        <FactorRow
            icon={<IconDeviceMobile />}
            name="Authenticator App"
            description="Time-based one-time passwords from your authenticator app."
            tag={{ label: isEnabled ? totp?.display_name ?? 'Enabled' : 'Disabled', enabled: isEnabled }}
            onToggle={handleToggle}
            open={step !== 'idle'}
        >
            <div className="ml-9 px-5">
                {step === 'idle' && isEnabled && (
                    <ExpandForm open>
                        <div className="pb-4 space-y-2">
                            <ErrorMsg msg={disableError} />
                            <button onClick={handleDisable} disabled={disable.isPending}
                                className="flex items-center gap-1 text-xs text-destructive/60 hover:text-destructive transition-colors disabled:opacity-50">
                                {disable.isPending ? 'Disabling…' : 'Disable'}
                            </button>
                        </div>
                    </ExpandForm>
                )}

                <ExpandForm open={step === 'name'}>
                    <Form {...nameForm}>
                        <form onSubmit={nameForm.handleSubmit(onNameSubmit)} className="space-y-3 max-w-sm pb-4">
                            <FormField control={nameForm.control} name="display_name" render={({ field }) => (
                                <FormItem className="space-y-1.5">
                                    <Label className="text-xs">Authenticator name</Label>
                                    <FormControl>
                                        <Input {...field} placeholder="Authy, Google Authenticator…" className="h-9 text-sm" maxLength={32} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )} />
                            <div className="flex gap-2">
                                <Button size="sm" type="submit" disabled={enable.isPending}>
                                    {enable.isPending ? 'Generating…' : 'Continue'}
                                </Button>
                                <Button size="sm" variant="ghost" type="button" onClick={() => { setStep('idle'); nameForm.reset(); }}>Cancel</Button>
                            </div>
                        </form>
                    </Form>
                </ExpandForm>

                <ExpandForm open={step === 'confirm' && !!setupData}>
                    {setupData && (
                        <div className="space-y-3 pb-4">
                            <div className="rounded-lg border border-border bg-muted/30 p-4 max-w-sm">
                                <p className="text-xs text-muted-foreground mb-3">Scan with your authenticator app, or enter the secret manually.</p>
                                <div className="flex justify-center mb-3">
                                    <div className="p-2 bg-white rounded-md">
                                        <QRCode value={setupData.qr} size={148} />
                                    </div>
                                </div>
                                <div>
                                    <div className="flex items-center justify-between mb-1">
                                        <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">Secret</span>
                                        <CopyButton text={setupData.secret} />
                                    </div>
                                    <code className="font-mono text-[11px] break-all text-foreground leading-relaxed">{setupData.secret}</code>
                                </div>
                            </div>
                            <Form {...codeForm}>
                                <form onSubmit={codeForm.handleSubmit(onCodeSubmit)} className="space-y-3 max-w-xs">
                                    <FormField control={codeForm.control} name="code" render={({ field }) => (
                                        <FormItem className="space-y-1.5">
                                            <Label className="text-xs">Verification code</Label>
                                            <FormControl>
                                                <Input
                                                    {...field}
                                                    onChange={e => field.onChange(e.target.value.replace(/\D/g, '').slice(0, 6))}
                                                    placeholder="000000"
                                                    className="h-9 font-mono tracking-[0.3em] text-sm text-center"
                                                    maxLength={6}
                                                    inputMode="numeric"
                                                    autoComplete="one-time-code"
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )} />
                                    <div className="flex gap-2">
                                        <Button size="sm" type="submit" disabled={confirm.isPending}>
                                            <IconCheck size={13} /> {confirm.isPending ? 'Verifying…' : 'Confirm'}
                                        </Button>
                                        <Button size="sm" variant="ghost" type="button" onClick={() => { setStep('idle'); setSetupData(null); codeForm.reset(); }}>Cancel</Button>
                                    </div>
                                </form>
                            </Form>
                        </div>
                    )}
                </ExpandForm>
            </div>
        </FactorRow>
    );
}
