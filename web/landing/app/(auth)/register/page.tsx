'use client'

import { useState } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/navigation'
import { motion } from 'framer-motion'
import { Mail, Lock, Eye, EyeOff, Loader2, User, Wallet, ArrowRight, Check, X } from 'lucide-react'
import { ConnectButton } from '@rainbow-me/rainbowkit'

// Social provider icons
const GoogleIcon = () => (
  <svg className="w-5 h-5" viewBox="0 0 24 24">
    <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
    <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
    <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
    <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
  </svg>
)

const DiscordIcon = () => (
  <svg className="w-5 h-5" viewBox="0 0 24 24" fill="#5865F2">
    <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"/>
  </svg>
)

const TwitchIcon = () => (
  <svg className="w-5 h-5" viewBox="0 0 24 24" fill="#9146FF">
    <path d="M11.571 4.714h1.715v5.143H11.57zm4.715 0H18v5.143h-1.714zM6 0L1.714 4.286v15.428h5.143V24l4.286-4.286h3.428L22.286 12V0zm14.571 11.143l-3.428 3.428h-3.429l-3 3v-3H6.857V1.714h13.714z"/>
  </svg>
)

const passwordRequirements = [
  { id: 'length', label: 'At least 8 characters', test: (p: string) => p.length >= 8 },
  { id: 'uppercase', label: 'One uppercase letter', test: (p: string) => /[A-Z]/.test(p) },
  { id: 'lowercase', label: 'One lowercase letter', test: (p: string) => /[a-z]/.test(p) },
  { id: 'number', label: 'One number', test: (p: string) => /\d/.test(p) },
]

export default function RegisterPage() {
  const router = useRouter()
  const [showPassword, setShowPassword] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [socialLoading, setSocialLoading] = useState<string | null>(null)
  const [step, setStep] = useState(1)
  const [formData, setFormData] = useState({
    email: '',
    accountName: '',
    password: '',
    confirmPassword: '',
    agreeToTerms: false,
    subscribeNewsletter: true,
  })

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (step === 1) {
      setStep(2)
      return
    }
    
    setIsLoading(true)
    await new Promise(resolve => setTimeout(resolve, 2000))
    router.push('/dashboard?welcome=true')
  }

  const handleSocialLogin = async (provider: 'google' | 'discord' | 'twitch') => {
    setSocialLoading(provider)
    
    // In production, redirect to OAuth endpoint
    await new Promise(resolve => setTimeout(resolve, 500))
    console.log(`Redirecting to ${provider} OAuth for registration...`)
    
    setSocialLoading(null)
  }

  const passwordStrength = passwordRequirements.filter(req => req.test(formData.password)).length

  const socialProviders = [
    { id: 'google', name: 'Google', Icon: GoogleIcon, color: 'hover:bg-white/10' },
    { id: 'discord', name: 'Discord', Icon: DiscordIcon, color: 'hover:bg-[#5865F2]/20' },
    { id: 'twitch', name: 'Twitch', Icon: TwitchIcon, color: 'hover:bg-[#9146FF]/20' },
  ] as const

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
    >
      {/* Mobile Logo */}
      <div className="lg:hidden flex justify-center mb-8">
        <Link href="/" className="flex items-center space-x-3">
          <div className="w-12 h-12 bg-gradient-to-br from-accent-500 to-accent-700 rounded-xl flex items-center justify-center">
            <span className="font-display font-bold text-2xl text-white">S</span>
          </div>
          <span className="font-display font-bold text-2xl">
            Shadow <span className="text-accent-500">OT</span>
          </span>
        </Link>
      </div>

      <div className="text-center mb-8">
        <h1 className="font-display text-3xl font-bold mb-2">Create Account</h1>
        <p className="text-shadow-400">Begin your journey in the Shadow Realms</p>
      </div>

      {/* Progress Steps */}
      <div className="flex items-center justify-center mb-8">
        <div className="flex items-center space-x-3">
          <div className={`w-8 h-8 rounded-full flex items-center justify-center font-medium text-sm ${
            step >= 1 ? 'bg-accent-500 text-white' : 'bg-shadow-800 text-shadow-400'
          }`}>
            {step > 1 ? <Check className="w-4 h-4" /> : '1'}
          </div>
          <div className={`w-16 h-1 rounded ${step > 1 ? 'bg-accent-500' : 'bg-shadow-700'}`} />
          <div className={`w-8 h-8 rounded-full flex items-center justify-center font-medium text-sm ${
            step >= 2 ? 'bg-accent-500 text-white' : 'bg-shadow-800 text-shadow-400'
          }`}>
            2
          </div>
        </div>
      </div>

      {/* Social Login Options - Step 1 only */}
      {step === 1 && (
        <>
          <div className="space-y-3 mb-6">
            {/* Wallet Connect */}
            <ConnectButton.Custom>
              {({ account, chain, openConnectModal, mounted }) => {
                const connected = mounted && account && chain
                return (
                  <button
                    onClick={openConnectModal}
                    className="w-full flex items-center justify-center space-x-3 py-3 px-4 bg-gradient-to-r from-purple-500/10 to-blue-500/10 hover:from-purple-500/20 hover:to-blue-500/20 border border-purple-500/30 rounded-xl transition-all duration-300 group"
                  >
                    <Wallet className="w-5 h-5 text-purple-400" />
                    <span className="text-white font-medium">
                      {connected ? `Connected: ${account.displayName}` : 'Register with Wallet'}
                    </span>
                    <ArrowRight className="w-4 h-4 text-purple-400 opacity-0 group-hover:opacity-100 transition-opacity" />
                  </button>
                )
              }}
            </ConnectButton.Custom>

            {/* Social Providers */}
            <div className="grid grid-cols-3 gap-3">
              {socialProviders.map(({ id, name, Icon, color }) => (
                <button
                  key={id}
                  onClick={() => handleSocialLogin(id)}
                  disabled={socialLoading !== null}
                  className={`flex items-center justify-center gap-2 py-3 px-4 bg-shadow-900 border border-shadow-700 rounded-xl transition-all duration-300 ${color} disabled:opacity-50 disabled:cursor-not-allowed`}
                >
                  {socialLoading === id ? (
                    <Loader2 className="w-5 h-5 animate-spin text-shadow-400" />
                  ) : (
                    <Icon />
                  )}
                  <span className="text-white text-sm font-medium hidden sm:inline">{name}</span>
                </button>
              ))}
            </div>
          </div>

          <div className="relative my-8">
            <div className="absolute inset-0 flex items-center">
              <div className="w-full border-t border-shadow-700"></div>
            </div>
            <div className="relative flex justify-center text-sm">
              <span className="px-4 bg-shadow-950 text-shadow-500">or register with email</span>
            </div>
          </div>
        </>
      )}

      <form onSubmit={handleSubmit} className="space-y-5">
        {step === 1 ? (
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            className="space-y-5"
          >
            {/* Email */}
            <div>
              <label htmlFor="email" className="block text-sm font-medium text-shadow-300 mb-2">
                Email Address
              </label>
              <div className="relative">
                <Mail className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
                <input
                  id="email"
                  type="email"
                  required
                  value={formData.email}
                  onChange={(e) => setFormData({ ...formData, email: e.target.value })}
                  className="w-full pl-12 pr-4 py-3 bg-shadow-900 border border-shadow-700 rounded-xl text-white placeholder-shadow-500 focus:outline-none focus:border-accent-500 focus:ring-1 focus:ring-accent-500 transition-all"
                  placeholder="warrior@shadowot.com"
                />
              </div>
            </div>

            {/* Account Name */}
            <div>
              <label htmlFor="accountName" className="block text-sm font-medium text-shadow-300 mb-2">
                Account Name
              </label>
              <div className="relative">
                <User className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
                <input
                  id="accountName"
                  type="text"
                  required
                  minLength={4}
                  maxLength={20}
                  value={formData.accountName}
                  onChange={(e) => setFormData({ ...formData, accountName: e.target.value })}
                  className="w-full pl-12 pr-4 py-3 bg-shadow-900 border border-shadow-700 rounded-xl text-white placeholder-shadow-500 focus:outline-none focus:border-accent-500 focus:ring-1 focus:ring-accent-500 transition-all"
                  placeholder="Choose a unique name"
                />
              </div>
              <p className="text-shadow-500 text-xs mt-1.5">4-20 characters, letters and numbers only</p>
            </div>

            {/* Newsletter */}
            <label className="flex items-start space-x-3 cursor-pointer">
              <input
                type="checkbox"
                checked={formData.subscribeNewsletter}
                onChange={(e) => setFormData({ ...formData, subscribeNewsletter: e.target.checked })}
                className="w-4 h-4 mt-0.5 bg-shadow-900 border border-shadow-700 rounded text-accent-500 focus:ring-accent-500 focus:ring-offset-0"
              />
              <span className="text-sm text-shadow-400">
                Subscribe to news, updates, and special events
              </span>
            </label>
          </motion.div>
        ) : (
          <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            className="space-y-5"
          >
            {/* Password */}
            <div>
              <label htmlFor="password" className="block text-sm font-medium text-shadow-300 mb-2">
                Password
              </label>
              <div className="relative">
                <Lock className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
                <input
                  id="password"
                  type={showPassword ? 'text' : 'password'}
                  required
                  value={formData.password}
                  onChange={(e) => setFormData({ ...formData, password: e.target.value })}
                  className="w-full pl-12 pr-12 py-3 bg-shadow-900 border border-shadow-700 rounded-xl text-white placeholder-shadow-500 focus:outline-none focus:border-accent-500 focus:ring-1 focus:ring-accent-500 transition-all"
                  placeholder="Create a strong password"
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-4 top-1/2 -translate-y-1/2 text-shadow-500 hover:text-white transition-colors"
                >
                  {showPassword ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
                </button>
              </div>
              
              {/* Password Strength */}
              <div className="mt-3">
                <div className="flex space-x-1 mb-2">
                  {[1, 2, 3, 4].map((level) => (
                    <div
                      key={level}
                      className={`h-1 flex-1 rounded-full transition-colors ${
                        passwordStrength >= level
                          ? passwordStrength <= 2 ? 'bg-red-500'
                          : passwordStrength === 3 ? 'bg-yellow-500'
                          : 'bg-green-500'
                          : 'bg-shadow-700'
                      }`}
                    />
                  ))}
                </div>
                <div className="grid grid-cols-2 gap-2">
                  {passwordRequirements.map((req) => (
                    <div key={req.id} className="flex items-center space-x-2 text-xs">
                      {req.test(formData.password) ? (
                        <Check className="w-3.5 h-3.5 text-green-500" />
                      ) : (
                        <X className="w-3.5 h-3.5 text-shadow-600" />
                      )}
                      <span className={req.test(formData.password) ? 'text-green-500' : 'text-shadow-500'}>
                        {req.label}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            </div>

            {/* Confirm Password */}
            <div>
              <label htmlFor="confirmPassword" className="block text-sm font-medium text-shadow-300 mb-2">
                Confirm Password
              </label>
              <div className="relative">
                <Lock className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
                <input
                  id="confirmPassword"
                  type={showPassword ? 'text' : 'password'}
                  required
                  value={formData.confirmPassword}
                  onChange={(e) => setFormData({ ...formData, confirmPassword: e.target.value })}
                  className={`w-full pl-12 pr-12 py-3 bg-shadow-900 border rounded-xl text-white placeholder-shadow-500 focus:outline-none focus:ring-1 transition-all ${
                    formData.confirmPassword && formData.password !== formData.confirmPassword
                      ? 'border-red-500 focus:border-red-500 focus:ring-red-500'
                      : formData.confirmPassword && formData.password === formData.confirmPassword
                      ? 'border-green-500 focus:border-green-500 focus:ring-green-500'
                      : 'border-shadow-700 focus:border-accent-500 focus:ring-accent-500'
                  }`}
                  placeholder="Confirm your password"
                />
                {formData.confirmPassword && (
                  <div className="absolute right-4 top-1/2 -translate-y-1/2">
                    {formData.password === formData.confirmPassword ? (
                      <Check className="w-5 h-5 text-green-500" />
                    ) : (
                      <X className="w-5 h-5 text-red-500" />
                    )}
                  </div>
                )}
              </div>
            </div>

            {/* Terms */}
            <label className="flex items-start space-x-3 cursor-pointer">
              <input
                type="checkbox"
                required
                checked={formData.agreeToTerms}
                onChange={(e) => setFormData({ ...formData, agreeToTerms: e.target.checked })}
                className="w-4 h-4 mt-0.5 bg-shadow-900 border border-shadow-700 rounded text-accent-500 focus:ring-accent-500 focus:ring-offset-0"
              />
              <span className="text-sm text-shadow-400">
                I agree to the{' '}
                <Link href="/terms" className="text-accent-500 hover:text-accent-400">Terms of Service</Link>
                {' '}and{' '}
                <Link href="/privacy" className="text-accent-500 hover:text-accent-400">Privacy Policy</Link>
              </span>
            </label>
          </motion.div>
        )}

        {/* Buttons */}
        <div className="flex space-x-3">
          {step === 2 && (
            <button
              type="button"
              onClick={() => setStep(1)}
              className="flex-1 btn-secondary"
            >
              Back
            </button>
          )}
          <button
            type="submit"
            disabled={isLoading || (step === 2 && (passwordStrength < 4 || formData.password !== formData.confirmPassword))}
            className="flex-1 btn-primary flex items-center justify-center space-x-2 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isLoading ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin" />
                <span>Creating Account...</span>
              </>
            ) : step === 1 ? (
              <span>Continue</span>
            ) : (
              <span>Create Account</span>
            )}
          </button>
        </div>
      </form>

      {/* Sign In Link */}
      <p className="text-center text-shadow-400 mt-8">
        Already have an account?{' '}
        <Link href="/login" className="text-accent-500 hover:text-accent-400 font-medium transition-colors">
          Sign in
        </Link>
      </p>
    </motion.div>
  )
}
