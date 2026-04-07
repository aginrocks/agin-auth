import { FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { useFormContext } from 'react-hook-form';
import { FormSchema } from '../page';

export function SignatureStep() {
    const form = useFormContext<FormSchema>();

    return (
        <div className="space-y-1.5">
            <label className="text-sm font-medium">
                <span className="text-muted-foreground mr-1.5">2.</span>
                Paste the signed message
            </label>
            <FormField
                control={form.control}
                name="pgp_signature"
                render={({ field }) => (
                    <FormItem>
                        <FormControl>
                            <textarea
                                {...field}
                                autoFocus
                                placeholder="-----BEGIN PGP SIGNED MESSAGE-----"
                                className="min-h-28 w-full resize-none rounded-md border border-input dark:bg-input/30 bg-transparent px-3 py-2.5 font-mono text-xs placeholder:text-muted-foreground shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                            />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                )}
            />
        </div>
    );
}
