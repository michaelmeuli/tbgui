(function() {
    var implementors = Object.fromEntries([["ecdsa",[["impl&lt;C&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"ecdsa/struct.Signature.html\" title=\"struct ecdsa::Signature\">Signature</a>&lt;C&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"ecdsa/trait.PrimeCurve.html\" title=\"trait ecdsa::PrimeCurve\">PrimeCurve</a>,\n    Self: <a class=\"trait\" href=\"const_oid/trait.AssociatedOid.html\" title=\"trait const_oid::AssociatedOid\">AssociatedOid</a>,</div>"],["impl&lt;C&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"ecdsa/struct.SigningKey.html\" title=\"struct ecdsa::SigningKey\">SigningKey</a>&lt;C&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"const_oid/trait.AssociatedOid.html\" title=\"trait const_oid::AssociatedOid\">AssociatedOid</a> + <a class=\"trait\" href=\"elliptic_curve/arithmetic/trait.CurveArithmetic.html\" title=\"trait elliptic_curve::arithmetic::CurveArithmetic\">CurveArithmetic</a> + <a class=\"trait\" href=\"ecdsa/trait.PrimeCurve.html\" title=\"trait ecdsa::PrimeCurve\">PrimeCurve</a>,\n    <a class=\"type\" href=\"elliptic_curve/scalar/type.Scalar.html\" title=\"type elliptic_curve::scalar::Scalar\">Scalar</a>&lt;C&gt;: <a class=\"trait\" href=\"elliptic_curve/ops/trait.Invert.html\" title=\"trait elliptic_curve::ops::Invert\">Invert</a>&lt;Output = <a class=\"struct\" href=\"subtle/struct.CtOption.html\" title=\"struct subtle::CtOption\">CtOption</a>&lt;<a class=\"type\" href=\"elliptic_curve/scalar/type.Scalar.html\" title=\"type elliptic_curve::scalar::Scalar\">Scalar</a>&lt;C&gt;&gt;&gt; + <a class=\"trait\" href=\"ecdsa/hazmat/trait.SignPrimitive.html\" title=\"trait ecdsa::hazmat::SignPrimitive\">SignPrimitive</a>&lt;C&gt;,\n    <a class=\"type\" href=\"ecdsa/type.SignatureSize.html\" title=\"type ecdsa::SignatureSize\">SignatureSize</a>&lt;C&gt;: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.0/std/primitive.u8.html\">u8</a>&gt;,</div>"],["impl&lt;C&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"ecdsa/struct.VerifyingKey.html\" title=\"struct ecdsa::VerifyingKey\">VerifyingKey</a>&lt;C&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"const_oid/trait.AssociatedOid.html\" title=\"trait const_oid::AssociatedOid\">AssociatedOid</a> + <a class=\"trait\" href=\"elliptic_curve/arithmetic/trait.CurveArithmetic.html\" title=\"trait elliptic_curve::arithmetic::CurveArithmetic\">CurveArithmetic</a> + <a class=\"trait\" href=\"ecdsa/trait.PrimeCurve.html\" title=\"trait ecdsa::PrimeCurve\">PrimeCurve</a>,\n    <a class=\"type\" href=\"elliptic_curve/point/type.AffinePoint.html\" title=\"type elliptic_curve::point::AffinePoint\">AffinePoint</a>&lt;C&gt;: <a class=\"trait\" href=\"elliptic_curve/sec1/trait.FromEncodedPoint.html\" title=\"trait elliptic_curve::sec1::FromEncodedPoint\">FromEncodedPoint</a>&lt;C&gt; + <a class=\"trait\" href=\"elliptic_curve/sec1/trait.ToEncodedPoint.html\" title=\"trait elliptic_curve::sec1::ToEncodedPoint\">ToEncodedPoint</a>&lt;C&gt;,\n    <a class=\"type\" href=\"elliptic_curve/field/type.FieldBytesSize.html\" title=\"type elliptic_curve::field::FieldBytesSize\">FieldBytesSize</a>&lt;C&gt;: <a class=\"trait\" href=\"sec1/point/trait.ModulusSize.html\" title=\"trait sec1::point::ModulusSize\">ModulusSize</a>,</div>"]]],["elliptic_curve",[["impl&lt;C&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"elliptic_curve/struct.PublicKey.html\" title=\"struct elliptic_curve::PublicKey\">PublicKey</a>&lt;C&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"const_oid/trait.AssociatedOid.html\" title=\"trait const_oid::AssociatedOid\">AssociatedOid</a> + <a class=\"trait\" href=\"elliptic_curve/trait.CurveArithmetic.html\" title=\"trait elliptic_curve::CurveArithmetic\">CurveArithmetic</a>,</div>"],["impl&lt;C&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"elliptic_curve/struct.SecretKey.html\" title=\"struct elliptic_curve::SecretKey\">SecretKey</a>&lt;C&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"const_oid/trait.AssociatedOid.html\" title=\"trait const_oid::AssociatedOid\">AssociatedOid</a> + <a class=\"trait\" href=\"elliptic_curve/trait.Curve.html\" title=\"trait elliptic_curve::Curve\">Curve</a>,</div>"]]],["rsa",[["impl&lt;D&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"rsa/pkcs1v15/struct.SigningKey.html\" title=\"struct rsa::pkcs1v15::SigningKey\">SigningKey</a>&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"digest/digest/trait.Digest.html\" title=\"trait digest::digest::Digest\">Digest</a>,</div>"],["impl&lt;D&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"rsa/pkcs1v15/struct.VerifyingKey.html\" title=\"struct rsa::pkcs1v15::VerifyingKey\">VerifyingKey</a>&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"digest/digest/trait.Digest.html\" title=\"trait digest::digest::Digest\">Digest</a>,</div>"],["impl&lt;D&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"rsa/pss/struct.BlindedSigningKey.html\" title=\"struct rsa::pss::BlindedSigningKey\">BlindedSigningKey</a>&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"digest/digest/trait.Digest.html\" title=\"trait digest::digest::Digest\">Digest</a>,</div>"],["impl&lt;D&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"rsa/pss/struct.SigningKey.html\" title=\"struct rsa::pss::SigningKey\">SigningKey</a>&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"digest/digest/trait.Digest.html\" title=\"trait digest::digest::Digest\">Digest</a>,</div>"],["impl&lt;D&gt; <a class=\"trait\" href=\"spki/traits/trait.AssociatedAlgorithmIdentifier.html\" title=\"trait spki::traits::AssociatedAlgorithmIdentifier\">AssociatedAlgorithmIdentifier</a> for <a class=\"struct\" href=\"rsa/pss/struct.VerifyingKey.html\" title=\"struct rsa::pss::VerifyingKey\">VerifyingKey</a>&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"digest/digest/trait.Digest.html\" title=\"trait digest::digest::Digest\">Digest</a>,</div>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[3862,1255,2413]}