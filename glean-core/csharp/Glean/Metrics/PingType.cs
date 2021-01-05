﻿// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.FFI;
using System;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Private
{
    /// <summary>
    ///  An enum with no values for convenient use as the default set of reason codes.
    /// </summary>
    public enum NoReasonCodes : int
    {
        value
    }

    /// <summary>
    ///  The base class of all PingTypes with just enough to track their registration, so
    ///  we can create a heterogeneous collection of ping types.
    /// </summary>
    public class PingTypeBase
    {
        internal string name;
        internal UInt64 handle;

        protected internal PingTypeBase() { }
    }

    /// <summary>
    ///  This implements the developer facing API for custom pings.
    ///  Instances of this class type are automatically generated by the parsers at build time.
    ///  The Ping API only exposes the [send] method, which schedules a ping for sending.
    /// </summary>
    public class PingType<ReasonCodesEnum> : PingTypeBase where ReasonCodesEnum : struct, Enum
    {
        private readonly string[] reasonCodes;

        public PingType (
            string name,
            bool includeClientId,
            bool sendIfEmpty,
            string[] reasonCodes
            )
        {
            handle = LibGleanFFI.glean_new_ping_type(
                name: name,
                include_client_id: Convert.ToByte(includeClientId),
                send_if_empty: Convert.ToByte(sendIfEmpty),
                reason: reasonCodes,
                reason_codes_len: reasonCodes == null ? 0 : reasonCodes.Length
            );

            this.name = name;
            this.reasonCodes = reasonCodes;

            GleanInstance.RegisterPingType(this);
        }

        /// <summary>
        ///  Collect and submit the ping for eventual upload.
        ///  
        ///  While the collection of metrics into pings happens synchronously, the
        ///  ping queuing and ping uploading happens asyncronously.
        ///  There are no guarantees that this will happen immediately.
        ///  
        ///  If the ping currently contains no content, it will not be queued.
        /// </summary>
        /// <param name="reason">The reason code enum for ping submit.</param>
        public void Submit(ReasonCodesEnum? reason = null)
        {
            int enumValue = Convert.ToInt32(reason.GetValueOrDefault());
            string reasonString =
                (reason != null && reasonCodes != null && reasonCodes.Length > 0) ? reasonCodes[enumValue] : null;
            GleanInstance.SubmitPing(this, reasonString);
        }
    }
}