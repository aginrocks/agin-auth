'use client';

import { useState, useEffect, useRef } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { Label } from '@components/ui/label';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@components/ui/dialog';
import { InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot } from '@components/ui/input-otp';
import { REGEXP_ONLY_DIGITS } from 'input-otp';
import { IconDeviceMobile } from '@tabler/icons-react';
import QRCode from 'react-qr-code';
import { FactorRow } from './factor-row';
import { CopyButton, ExpandForm } from './helpers';
import { FactorKeyItem } from './factor-key-item';
import { Dialog as ConfirmDialog, DialogContent as ConfirmDialogContent, DialogDescription as ConfirmDialogDescription, DialogFooter as ConfirmDialogFooter, DialogHeader as ConfirmDialogHeader, DialogTitle as ConfirmDialogTitle } from '@components/ui/dialog';

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
    const [confirmDisable, setConfirmDisable] = useState(false);
    const [detailsOpen, setDetailsOpen] = useState(false);
    const lastSubmittedCode = useRef<string | null>(null);

    const isEnabled = totp?.fully_enabled ?? false;

    const nameForm = useForm<NameForm>({
        resolver: zodResolver(nameSchema),
        defaultValues: { display_name: '' },
    });

    const codeForm = useForm<CodeForm>({
        resolver: zodResolver(codeSchema),
        defaultValues: { code: '' },
    });

    const enable = $api.useMutation('post', '/api/settings/factors/totp/enable', {
        onSuccess: (data) => {
            setSetupData(data);
            setStep('confirm');
        },
        onError: () => {
            nameForm.setError('display_name', { message: 'Failed to start setup.' });
        },
    });

    const confirmMutation = $api.useMutation('post', '/api/settings/factors/totp/enable/confirm', {
        onSuccess: () => {
            setStep('idle');
            setSetupData(null);
            nameForm.reset();
            codeForm.reset();
            onRefetch();
        },
        onError: () => {
            codeForm.setError('code', { message: 'Invalid code, try again.' });
        },
    });

    const disable = $api.useMutation('delete', '/api/settings/factors/totp/disable', {
        onSuccess: () => { setConfirmDisable(false); onRefetch(); },
    });

    const handleToggle = () => {
        if (step !== 'idle') {
            setStep('idle');
            setSetupData(null);
            nameForm.reset();
            codeForm.reset();
            lastSubmittedCode.current = null;
        } else if (isEnabled) {
            setDetailsOpen(v => !v);
        } else {
            setStep('name');
        }
    };

    const code = codeForm.watch('code');

    useEffect(() => {
        if (code?.length !== 6) {
            lastSubmittedCode.current = null;
            return;
        }

        if (confirmMutation.isPending || lastSubmittedCode.current === code) {
            return;
        }

        lastSubmittedCode.current = code;
        confirmMutation.mutate({ body: { code } });
    }, [code, confirmMutation]);

    const closeSetupDialog = () => {
        setStep('idle');
        setSetupData(null);
        codeForm.reset();
        lastSubmittedCode.current = null;
    };

    return (
        <>
            <FactorRow
                icon={<IconDeviceMobile />}
                name="Authenticator App"
                description="Time-based one-time passwords from your authenticator app."
                tag={{ label: isEnabled ? totp?.display_name ?? 'Enabled' : 'Disabled', enabled: isEnabled }}
                onToggle={handleToggle}
                open={step !== 'idle' || detailsOpen}
            >
                <div className="ml-9 px-5">
                    {isEnabled && (
                        <ExpandForm open={detailsOpen}>
                            <div className="pb-3 max-w-sm">
                                <FactorKeyItem
                                    icon={<IconDeviceMobile size={14} className="text-muted-foreground" />}
                                    name={totp?.display_name ?? 'Authenticator'}
                                    subtitle="Active"
                                    onRemove={() => setConfirmDisable(true)}
                                />
                            </div>
                        </ExpandForm>
                    )}

                    <ExpandForm open={step === 'name'}>
                        <Form {...nameForm}>
                            <form onSubmit={nameForm.handleSubmit(data => enable.mutate({ body: data }))} className="space-y-3 max-w-sm pb-4">
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
                </div>
            </FactorRow>

            <Dialog open={step === 'confirm' && !!setupData} onOpenChange={(open) => { if (!open) closeSetupDialog(); }}>
                <DialogContent className="sm:max-w-md">
                    <DialogHeader>
                        <DialogTitle>Set up authenticator</DialogTitle>
                        <DialogDescription>
                            Scan the QR code with your authenticator app, then enter the verification code.
                        </DialogDescription>
                    </DialogHeader>
                    {setupData && (
                        <div className="space-y-4">
                            <div className="flex justify-center">
                                <div className="p-3 bg-white rounded-lg">
                                    <QRCode value={setupData.qr} size={160} />
                                </div>
                            </div>
                            <div className="rounded-lg border border-border bg-muted/30 p-3">
                                <div className="flex items-center justify-between mb-1">
                                    <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">Secret</span>
                                    <CopyButton text={setupData.secret} />
                                </div>
                                <code className="font-mono text-[11px] break-all text-foreground leading-relaxed">{setupData.secret}</code>
                            </div>

                            <Form {...codeForm}>
                                <form onSubmit={codeForm.handleSubmit(data => confirmMutation.mutate({ body: data }))}>
                                    <FormField control={codeForm.control} name="code" render={({ field }) => (
                                        <FormItem className="flex flex-col items-center gap-2">
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
                                    )} />
                                </form>
                            </Form>

                            {confirmMutation.isPending && (
                                <p className="text-xs text-muted-foreground text-center">Verifying…</p>
                            )}
                        </div>
                    )}
                </DialogContent>
            </Dialog>

            <ConfirmDialog open={confirmDisable} onOpenChange={setConfirmDisable}>
                <ConfirmDialogContent>
                    <ConfirmDialogHeader>
                        <ConfirmDialogTitle>Remove authenticator</ConfirmDialogTitle>
                        <ConfirmDialogDescription>
                            This will remove{' '}<span className="font-medium text-foreground">{totp?.display_name}</span>{' '}from your account. You won&apos;t be able to use it for two-factor authentication until you set up a new one.
                        </ConfirmDialogDescription>
                    </ConfirmDialogHeader>
                    {disable.isError && (
                        <p className="text-xs text-destructive">Failed to remove authenticator.</p>
                    )}
                    <ConfirmDialogFooter>
                        <Button variant="outline" onClick={() => setConfirmDisable(false)} disabled={disable.isPending}>
                            Cancel
                        </Button>
                        <Button variant="destructive" onClick={() => disable.mutate({})} disabled={disable.isPending}>
                            {disable.isPending ? 'Removing…' : 'Remove'}
                        </Button>
                    </ConfirmDialogFooter>
                </ConfirmDialogContent>
            </ConfirmDialog>
        </>
    );
}
