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
import { IconDeviceMobile } from '@tabler/icons-react';
import { FactorRow } from './factor-row';
import { ExpandForm } from './helpers';
import { FactorKeyItem } from './factor-key-item';
import { TotpSetupDialog } from './totp-setup-dialog';
import { TotpDisableDialog } from './totp-disable-dialog';

const nameSchema = z.object({
    display_name: z.string().min(1, 'Required').max(32),
});
const codeSchema = z.object({
    code: z.string().length(6, 'Must be 6 digits'),
});

type NameForm = z.infer<typeof nameSchema>;
type CodeForm = z.infer<typeof codeSchema>;

export function TotpRow({
    totp,
    onRefetch,
}: {
    totp: { display_name: string; fully_enabled: boolean } | null | undefined;
    onRefetch: () => void;
}) {
    const [step, setStep] = useState<'idle' | 'name' | 'confirm'>('idle');
    const [setupData, setSetupData] = useState<{ secret: string; qr: string } | null>(null);
    const [setupDialogOpen, setSetupDialogOpen] = useState(false);
    const [confirmDisable, setConfirmDisable] = useState(false);
    const [disableDisplayName, setDisableDisplayName] = useState('');
    const [detailsOpen, setDetailsOpen] = useState(false);
    const lastSubmittedCode = useRef<string | null>(null);
    const setupDialogResetTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const disableDialogResetTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

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
            if (setupDialogResetTimeoutRef.current) {
                clearTimeout(setupDialogResetTimeoutRef.current);
                setupDialogResetTimeoutRef.current = null;
            }
            setSetupData(data);
            setStep('confirm');
            setSetupDialogOpen(true);
        },
        onError: () => {
            nameForm.setError('display_name', { message: 'Failed to start setup.' });
        },
    });

    const confirmMutation = $api.useMutation('post', '/api/settings/factors/totp/enable/confirm', {
        onSuccess: () => {
            closeSetupDialog();
            onRefetch();
        },
        onError: () => {
            codeForm.setError('code', { message: 'Invalid code, try again.' });
        },
    });

    const disable = $api.useMutation('delete', '/api/settings/factors/totp/disable', {
        onSuccess: () => {
            closeDisableDialog();
            onRefetch();
        },
    });

    const resetSetupState = () => {
        if (setupDialogResetTimeoutRef.current) {
            clearTimeout(setupDialogResetTimeoutRef.current);
            setupDialogResetTimeoutRef.current = null;
        }

        setStep('idle');
        setSetupData(null);
        setSetupDialogOpen(false);
        nameForm.reset();
        codeForm.reset();
        lastSubmittedCode.current = null;
    };

    const handleToggle = () => {
        if (step === 'confirm') {
            closeSetupDialog();
        } else if (step !== 'idle') {
            resetSetupState();
        } else if (isEnabled) {
            setDetailsOpen((v) => !v);
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
        if (setupDialogResetTimeoutRef.current) {
            clearTimeout(setupDialogResetTimeoutRef.current);
        }

        setSetupDialogOpen(false);
        setupDialogResetTimeoutRef.current = setTimeout(() => {
            resetSetupState();
        }, 200);
    };

    const closeDisableDialog = () => {
        if (disableDialogResetTimeoutRef.current) {
            clearTimeout(disableDialogResetTimeoutRef.current);
        }

        setConfirmDisable(false);
        disableDialogResetTimeoutRef.current = setTimeout(() => {
            setDisableDisplayName('');
            disableDialogResetTimeoutRef.current = null;
        }, 200);
    };

    useEffect(() => {
        return () => {
            if (setupDialogResetTimeoutRef.current) {
                clearTimeout(setupDialogResetTimeoutRef.current);
            }
            if (disableDialogResetTimeoutRef.current) {
                clearTimeout(disableDialogResetTimeoutRef.current);
            }
        };
    }, []);

    return (
        <>
            <FactorRow
                icon={<IconDeviceMobile />}
                name="Authenticator App"
                description="Time-based one-time passwords from your authenticator app."
                tag={{
                    label: isEnabled ? (totp?.display_name ?? 'Enabled') : 'Disabled',
                    enabled: isEnabled,
                }}
                onToggle={handleToggle}
                open={step === 'name' || detailsOpen}
            >
                <div className="ml-9 px-5">
                    {isEnabled && (
                        <ExpandForm open={detailsOpen}>
                            <div className="pb-3 max-w-sm">
                                <FactorKeyItem
                                    icon={
                                        <IconDeviceMobile
                                            size={14}
                                            className="text-muted-foreground"
                                        />
                                    }
                                    name={totp?.display_name ?? 'Authenticator'}
                                    subtitle="Active"
                                    onRemove={() => {
                                        if (disableDialogResetTimeoutRef.current) {
                                            clearTimeout(disableDialogResetTimeoutRef.current);
                                            disableDialogResetTimeoutRef.current = null;
                                        }
                                        setDisableDisplayName(
                                            totp?.display_name ?? 'Authenticator'
                                        );
                                        setConfirmDisable(true);
                                    }}
                                />
                            </div>
                        </ExpandForm>
                    )}

                    <ExpandForm open={step === 'name'}>
                        <Form {...nameForm}>
                            <form
                                onSubmit={nameForm.handleSubmit((data) =>
                                    enable.mutate({ body: data })
                                )}
                                className="space-y-3 max-w-sm pb-4"
                            >
                                <FormField
                                    control={nameForm.control}
                                    name="display_name"
                                    render={({ field }) => (
                                        <FormItem className="space-y-1.5">
                                            <Label className="text-xs">Authenticator name</Label>
                                            <FormControl>
                                                <Input
                                                    {...field}
                                                    placeholder="Authy, Google Authenticator…"
                                                    className="h-9 text-sm"
                                                    maxLength={32}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <div className="flex gap-2">
                                    <Button size="sm" type="submit" disabled={enable.isPending}>
                                        {enable.isPending ? 'Generating…' : 'Continue'}
                                    </Button>
                                    <Button
                                        size="sm"
                                        variant="ghost"
                                        type="button"
                                        onClick={() => {
                                            setStep('idle');
                                            nameForm.reset();
                                        }}
                                    >
                                        Cancel
                                    </Button>
                                </div>
                            </form>
                        </Form>
                    </ExpandForm>
                </div>
            </FactorRow>

            <TotpSetupDialog
                open={step === 'confirm' && !!setupData && setupDialogOpen}
                onOpenChange={(open) => {
                    if (open) {
                        if (setupDialogResetTimeoutRef.current) {
                            clearTimeout(setupDialogResetTimeoutRef.current);
                            setupDialogResetTimeoutRef.current = null;
                        }
                        setSetupDialogOpen(true);
                        return;
                    }
                    closeSetupDialog();
                }}
                setupData={setupData}
                codeForm={codeForm}
                onSubmit={(data) => confirmMutation.mutate({ body: data })}
            />

            <TotpDisableDialog
                open={confirmDisable}
                onOpenChange={(open) => {
                    if (open) {
                        if (disableDialogResetTimeoutRef.current) {
                            clearTimeout(disableDialogResetTimeoutRef.current);
                            disableDialogResetTimeoutRef.current = null;
                        }
                        setConfirmDisable(true);
                        return;
                    }
                    closeDisableDialog();
                }}
                displayName={disableDisplayName || totp?.display_name || 'Authenticator'}
                isLoading={disable.isPending}
                isError={disable.isError}
                onCancel={closeDisableDialog}
                onConfirm={() => disable.mutate({})}
            />
        </>
    );
}
