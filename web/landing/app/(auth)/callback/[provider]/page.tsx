'use client'

import { useEffect, useState } from 'react'
import { useParams, useSearchParams, useRouter } from 'next/navigation'
import { motion } from 'framer-motion'
import { Loader2, CheckCircle, XCircle, ArrowRight } from 'lucide-react'
import Link from 'next/link'

type OAuthProvider = 'google' | 'discord' | 'twitch'

const providerNames: Record<OAuthProvider, string> = {
  google: 'Google',
  discord: 'Discord',
  twitch: 'Twitch',
}

const providerColors: Record<OAuthProvider, string> = {
  google: '#4285F4',
  discord: '#5865F2',
  twitch: '#9146FF',
}

export default function OAuthCallbackPage() {
  const params = useParams()
  const searchParams = useSearchParams()
  const router = useRouter()
  const [status, setStatus] = useState<'loading' | 'success' | 'error'>('loading')
  const [error, setError] = useState<string | null>(null)
  const [redirectTo, setRedirectTo] = useState('/dashboard')

  const provider = params.provider as OAuthProvider
  const code = searchParams.get('code')
  const state = searchParams.get('state')
  const errorParam = searchParams.get('error')
  const errorDescription = searchParams.get('error_description')

  useEffect(() => {
    const handleCallback = async () => {
      // Handle OAuth error from provider
      if (errorParam) {
        setStatus('error')
        setError(errorDescription || `Authentication with ${providerNames[provider]} was denied`)
        return
      }

      // Validate required params
      if (!code || !state) {
        setStatus('error')
        setError('Invalid OAuth callback - missing parameters')
        return
      }

      try {
        // Get stored state for verification
        const storedStateJson = sessionStorage.getItem(`oauth_state_${state}`)
        if (!storedStateJson) {
          throw new Error('Invalid OAuth state - possible CSRF attack')
        }

        const storedState = JSON.parse(storedStateJson)
        
        // Check state expiration (5 minutes)
        if (Date.now() - storedState.timestamp > 5 * 60 * 1000) {
          throw new Error('OAuth session expired. Please try again.')
        }

        // Clean up stored state
        sessionStorage.removeItem(`oauth_state_${state}`)

        // Exchange code for tokens via backend
        const apiUrl = process.env.NEXT_PUBLIC_API_URL || ''
        const response = await fetch(`${apiUrl}/auth/oauth/${provider}/callback`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ code, state }),
        })

        if (!response.ok) {
          const data = await response.json().catch(() => ({}))
          throw new Error(data.message || 'Authentication failed')
        }

        const data = await response.json()
        
        // Store tokens
        if (data.tokens) {
          localStorage.setItem('accessToken', data.tokens.accessToken)
          localStorage.setItem('refreshToken', data.tokens.refreshToken)
        }

        setStatus('success')
        setRedirectTo(storedState.redirectAfter || '/dashboard')

        // Redirect after success animation
        setTimeout(() => {
          router.push(storedState.redirectAfter || '/dashboard')
        }, 1500)
      } catch (err: any) {
        setStatus('error')
        setError(err.message || 'Authentication failed')
      }
    }

    handleCallback()
  }, [code, state, errorParam, errorDescription, provider, router])

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.3 }}
      className="text-center"
    >
      {/* Provider Logo */}
      <div 
        className="w-20 h-20 rounded-2xl mx-auto mb-8 flex items-center justify-center"
        style={{ backgroundColor: `${providerColors[provider]}20` }}
      >
        {status === 'loading' ? (
          <Loader2 
            className="w-10 h-10 animate-spin" 
            style={{ color: providerColors[provider] }}
          />
        ) : status === 'success' ? (
          <CheckCircle className="w-10 h-10 text-green-400" />
        ) : (
          <XCircle className="w-10 h-10 text-red-400" />
        )}
      </div>

      {/* Status Text */}
      <h1 className="font-display text-2xl font-bold mb-2">
        {status === 'loading' && `Signing in with ${providerNames[provider]}...`}
        {status === 'success' && 'Welcome to Shadow OT!'}
        {status === 'error' && 'Authentication Failed'}
      </h1>

      <p className="text-shadow-400 mb-8">
        {status === 'loading' && 'Please wait while we verify your account...'}
        {status === 'success' && 'Redirecting you to your dashboard...'}
        {status === 'error' && (error || 'Something went wrong. Please try again.')}
      </p>

      {/* Actions */}
      {status === 'success' && (
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
        >
          <Link 
            href={redirectTo}
            className="inline-flex items-center gap-2 btn-primary"
          >
            Continue to Dashboard
            <ArrowRight className="w-4 h-4" />
          </Link>
        </motion.div>
      )}

      {status === 'error' && (
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
          className="space-y-4"
        >
          <Link href="/login" className="inline-block btn-primary">
            Back to Login
          </Link>
          <p className="text-shadow-500 text-sm">
            Need help?{' '}
            <Link href="/support" className="text-accent-500 hover:text-accent-400">
              Contact Support
            </Link>
          </p>
        </motion.div>
      )}

      {/* Loading Progress */}
      {status === 'loading' && (
        <motion.div
          initial={{ width: 0 }}
          animate={{ width: '100%' }}
          transition={{ duration: 2 }}
          className="h-1 bg-accent-500 rounded-full max-w-xs mx-auto"
        />
      )}
    </motion.div>
  )
}

