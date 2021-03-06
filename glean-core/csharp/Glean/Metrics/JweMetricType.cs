// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.FFI;
using System;
using System.Text.Json;

namespace Mozilla.Glean.Private
{
    /// <summary>
    ///  A representation of a JWE value.
    /// </summary>
    public readonly struct JweData
    {
        public JweData(string header, string key, string initVector, string cipherText, string authTag)
        {
            Header = header;
            Key = key;
            InitVector = initVector;
            CipherText = cipherText;
            AuthTag = authTag;
        }

        public string Header { get; }
        public string Key { get; }
        public string InitVector { get; }
        public string CipherText { get; }
        public string AuthTag { get; }
    }

    /// <summary>
    /// This implements the developer facing API for recording string metrics.
    /// 
    /// Instances of this class type are automatically generated by the parsers at build time,
    /// allowing developers to record values that were previously registered in the metrics.yaml file.
    /// 
    /// The internal constructor is only used by `LabeledMetricType` directly.
    /// </summary>
    public sealed class JweMetricType
    {
        private readonly bool disabled;
        private readonly string[] sendInPings;
        private readonly UInt64 handle;

        public JweMetricType(
            bool disabled,
            string category,
            Lifetime lifetime,
            string name,
            string[] sendInPings
            ) : this(0, disabled, sendInPings)
        {
            handle = LibGleanFFI.glean_new_jwe_metric(
                    category: category,
                    name: name,
                    send_in_pings: sendInPings,
                    send_in_pings_len: sendInPings.Length,
                    lifetime: (int)lifetime,
                    disabled: disabled);
        }

        internal JweMetricType(
            UInt64 handle,
            bool disabled,
            string[] sendInPings
            )
        {
            this.disabled = disabled;
            this.sendInPings = sendInPings;
            this.handle = handle;
        }

        /// <summary>
        /// Set a JWE value.
        /// </summary>
        /// <param name="value"> The [`compact representation`](https://tools.ietf.org/html/rfc7516#appendix-A.2.7) of a JWE value.</param>
        public void setWithCompactRepresentation(string value)
        {
            if (disabled)
            {
                return;
            }

            Dispatchers.LaunchAPI(() => {
                LibGleanFFI.glean_jwe_set_with_compact_representation(this.handle, value);
            });
        }

        /// <summary>
        /// Build a JWE value from it's elements and set to it.
        /// </summary>
        /// <param name="header">A variable-size JWE protected header.</param>
        /// <param name="key">A variable-size encrypted key.
        /// This can be an empty octet sequence.</param>
        /// <param name="initVector">A fixed-size, 96-bit, base64 encoded Jwe initialization vector.
        /// If not required by the encryption algorithm, can be an empty octet sequence.</param>
        /// <param name="cipherText">The variable-size base64 encoded cipher text.</param>
        /// <param name="authTag">A fixed-size, 132-bit, base64 encoded authentication tag.
        /// Can be an empty octet sequence.</param>
        public void Set(string header, string key, string initVector, string cipherText, string authTag)
        {
            if (disabled)
            {
                return;
            }

            Dispatchers.LaunchAPI(() => {
                LibGleanFFI.glean_jwe_set(this.handle, header, key, initVector, cipherText, authTag);
            });
        }


        /// <summary>
        /// Tests whether a value is stored for the metric for testing purposes only. This function will
        /// attempt to await the last task (if any) writing to the the metric's storage engine before
        /// returning a value.
        /// </summary>
        /// <param name="pingName">represents the name of the ping to retrieve the metric for Defaults
        /// to the first value in `sendInPings`</param>
        /// <returns>true if metric value exists, otherwise false</returns>
        public bool TestHasValue(string pingName = null)
        {
            Dispatchers.AssertInTestingMode();

            string ping = pingName ?? sendInPings[0];
            return LibGleanFFI.glean_jwe_test_has_value(this.handle, ping) != 0;
        }

        /// <summary>
        /// Returns the stored value for testing purposes only. This function will attempt to await the
        /// last task (if any) writing to the the metric's storage engine before returning a value.
        /// @throws [NullPointerException] if no value is stored
        /// </summary>
        /// <param name="pingName">represents the name of the ping to retrieve the metric for.
        /// Defaults to the first value in `sendInPings`</param>
        /// <returns>value of the stored metric</returns>
        /// <exception cref="System.NullReferenceException">Thrown when the metric contains no value</exception>
        public JweData TestGetValue(string pingName = null)
        {
            Dispatchers.AssertInTestingMode();

            if (!TestHasValue(pingName)) {
                throw new NullReferenceException();
            }

            string ping = pingName ?? sendInPings[0];

            JsonDocument jsonPayload = JsonDocument.Parse(
                LibGleanFFI.glean_jwe_test_get_value_as_json_string(this.handle, ping).AsString()
            );
            JsonElement root = jsonPayload.RootElement;
            return new JweData(
                root.GetProperty("header").GetString(),
                root.GetProperty("key").GetString(),
                root.GetProperty("init_vector").GetString(),
                root.GetProperty("cipher_text").GetString(),
                root.GetProperty("auth_tag").GetString()
            );
        }

        /// <summary>
        /// Returns the stored value in the compact representation for testing purposes only.
        /// This function will attempt to await the last task (if any)
        /// writing to the metric's storage engine before returning a value.
        /// @throws [NullPointerException] if no value is stored
        /// </summary>
        /// <param name="pingName">represents the name of the ping to retrieve the metric for.
        /// Defaults to the first value in `sendInPings`</param>
        /// <returns>value of the stored metric</returns>
        /// <exception cref="System.NullReferenceException">Thrown when the metric contains no value</exception>
        public string testGetCompactRepresentation(string pingName = null)
        {
            Dispatchers.AssertInTestingMode();

            if (!TestHasValue(pingName)) {
                throw new NullReferenceException();
            }

            string ping = pingName ?? sendInPings[0];
            return LibGleanFFI.glean_jwe_test_get_value(this.handle, ping).AsString();
        }

        /**
         * Returns the number of errors recorded for the given metric.
         *
         * @param errorType The type of the error recorded.
         * @param pingName represents the name of the ping to retrieve the metric for.
         *                 Defaults to the first value in `sendInPings`.
         * @return the number of errors recorded for the metric.
         */
        public Int32 TestGetNumRecordedErrors(Testing.ErrorType errorType, string pingName = null)
        {
            Dispatchers.AssertInTestingMode();

            string ping = pingName ?? sendInPings[0];
            return LibGleanFFI.glean_jwe_test_get_num_recorded_errors(
                this.handle, (int)errorType, ping
            );
        }
    }
}
