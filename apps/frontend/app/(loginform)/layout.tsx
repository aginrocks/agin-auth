import Image from 'next/image';

export default function LoginLayout({ children }: { children: React.ReactNode }) {
    return (
        <div className="flex h-screen p-2">
            <div className="flex-1 max-w-xl flex flex-col justify-center items-center">
                {children}
            </div>
            <div className="flex-1 relative">
                <Image src="/background.jpg" alt="" fill className="object-cover rounded-xl" />
            </div>
        </div>
    );
}
