'use client';

import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@components/ui/form';
import { Input } from '@components/ui/input';
import { LoginIcon } from '@components/ui/login-icon';
import { IconArrowRight, IconKey } from '@tabler/icons-react';
import { useFormContext } from 'react-hook-form';
import { FormSchema } from './layout';
import { Button } from '@components/ui/button';

export default function Page() {
    const form = useFormContext<FormSchema>();

    return (
        <Form {...form}>
            <LoginIcon>
                <IconKey />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Sign in to Agin</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Welcome back! Please sign in to continue
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-4">
                <FormField
                    control={form.control}
                    name="username"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Username or Email</FormLabel>
                            <FormControl>
                                <Input placeholder="Enter your username or email" {...field} />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button>
                    Next <IconArrowRight />
                </Button>
            </div>
        </Form>
    );
}
