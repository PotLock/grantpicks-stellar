import type { Metadata } from 'next'
import { Titillium_Web } from 'next/font/google'
import './globals.css'
import '@near-wallet-selector/modal-ui/styles.css'

const titiliumWeb = Titillium_Web({
	subsets: ['latin'],
	display: 'swap',
	variable: '--font-titilium-web',
	weight: ['200', '300', '400', '600', '700', '900'],
})
export const metadata: Metadata = {
	title: 'Create Next App',
	description: 'Generated by create next app',
}

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode
}>) {
	return (
		<html lang="en">
			<body className={titiliumWeb.className}>{children}</body>
		</html>
	)
}
